[
    (
        FugitivePath(`mnist_classifier::digits::zero::open_one_match`, `Val`),
        None,
    ),
    (
        FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
        Some(
            KiReprExpansion {
                hir_lazy_variable_ki_repr_map: ArenaMap {
                    data: [
                        Some(
                            KiRepr(
                                Id {
                                    value: 377,
                                },
                            ),
                        ),
                        Some(
                            KiRepr(
                                Id {
                                    value: 400,
                                },
                            ),
                        ),
                        Some(
                            KiRepr(
                                Id {
                                    value: 410,
                                },
                            ),
                        ),
                        Some(
                            KiRepr(
                                Id {
                                    value: 440,
                                },
                            ),
                        ),
                        Some(
                            KiRepr(
                                Id {
                                    value: 448,
                                },
                            ),
                        ),
                        Some(
                            KiRepr(
                                Id {
                                    value: 454,
                                },
                            ),
                        ),
                        Some(
                            KiRepr(
                                Id {
                                    value: 456,
                                },
                            ),
                        ),
                        Some(
                            KiRepr(
                                Id {
                                    value: 462,
                                },
                            ),
                        ),
                    ],
                },
                hir_lazy_expr_ki_repr_map: [
                    KiRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorVal {
                                    path: FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: KiReprSource::ValItem(
                            FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                        ),
                        caching_class: ValItem,
                    },
                    KiRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::connected_component::ConnectedComponent(0)::raw_contours`, `MemoizedField`),
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
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
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
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MethodRitchie {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(`(core::vec::Vec(0)::ilen`, `MethodRitchie(
                                            Fn,
                                        )`),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [
                                            (
                                                Type(
                                                    Type {
                                                        attrs: HirTemplateSvarAttrs {
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
                                                                    value: 12,
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
                                        value: 372,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 3,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Literal(
                            Literal::I32(
                                1,
                            ),
                        ),
                        arguments: [],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
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
                        val_domain_repr: Omni,
                        opn: ValOpn::Binary(
                            Comparison(
                                Eq,
                            ),
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 373,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 374,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
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
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorVal {
                                    path: FugitivePath(`mnist_classifier::digits::zero::open_one_match`, `Val`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: KiReprSource::ValItem(
                            FugitivePath(`mnist_classifier::digits::zero::open_one_match`, `Val`),
                        ),
                        caching_class: ValItem,
                    },
                    KiRepr {
                        val_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 375,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::fermi::FermiMatchResult(0)::norm`, `MemoizedField`),
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
                                        value: 370,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 7,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 375,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::fermi::FermiMatchResult(0)::norm`, `MemoizedField`),
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
                                        value: 370,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: LetVariable {
                                stmt: 1,
                            },
                        },
                        caching_class: Variable,
                    },
                    KiRepr {
                        val_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 375,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            Literal::F32(
                                TermF32Literal {
                                    value: OrderedFloat(
                                        1.5,
                                    ),
                                    text: "1.5f32",
                                },
                            ),
                        ),
                        arguments: [],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
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
                        val_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 375,
                                },
                            ),
                        ),
                        opn: ValOpn::Binary(
                            Comparison(
                                Less,
                            ),
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 377,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 379,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
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
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorVal {
                                    path: FugitivePath(`mnist_classifier::digits::zero::open_one_match`, `Val`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: KiReprSource::ValItem(
                            FugitivePath(`mnist_classifier::digits::zero::open_one_match`, `Val`),
                        ),
                        caching_class: ValItem,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 381,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
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
                                        value: 370,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
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
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 381,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            Literal::USize(
                                TermUSizeLiteral {
                                    value: 0,
                                },
                            ),
                        ),
                        arguments: [],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
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
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 381,
                                },
                            ),
                        ),
                        opn: ValOpn::Index,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 383,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 384,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
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
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorVal {
                                    path: FugitivePath(`mnist_classifier::major::connected_components`, `Val`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: KiReprSource::ValItem(
                            FugitivePath(`mnist_classifier::major::connected_components`, `Val`),
                        ),
                        caching_class: ValItem,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 387,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MethodRitchie {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(`(core::vec::Vec(0)::ilen`, `MethodRitchie(
                                            Fn,
                                        )`),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [
                                            (
                                                Type(
                                                    Type {
                                                        attrs: HirTemplateSvarAttrs {
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
                                                                    value: 13,
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
                                        value: 389,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 16,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 387,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            Literal::I32(
                                1,
                            ),
                        ),
                        arguments: [],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
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
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 387,
                                },
                            ),
                        ),
                        opn: ValOpn::Binary(
                            Comparison(
                                Eq,
                            ),
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 390,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 391,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
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
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorVal {
                                    path: FugitivePath(`mnist_classifier::digits::zero::open_one_match`, `Val`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: KiReprSource::ValItem(
                            FugitivePath(`mnist_classifier::digits::zero::open_one_match`, `Val`),
                        ),
                        caching_class: ValItem,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 393,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
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
                                        value: 370,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
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
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 393,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            Literal::USize(
                                TermUSizeLiteral {
                                    value: 0,
                                },
                            ),
                        ),
                        arguments: [],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 21,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 393,
                                },
                            ),
                        ),
                        opn: ValOpn::Index,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 394,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 395,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
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
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 393,
                                },
                            ),
                        ),
                        opn: ValOpn::Unwrap,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 396,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
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
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 393,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MethodRitchie {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::displacement`, `MethodRitchie(
                                            Fn,
                                        )`),
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
                                        value: 397,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
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
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 393,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MethodRitchie {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::geom2d::Vector2d(0)::norm`, `MethodRitchie(
                                            Fn,
                                        )`),
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
                                        value: 398,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 25,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 393,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MethodRitchie {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::geom2d::Vector2d(0)::norm`, `MethodRitchie(
                                            Fn,
                                        )`),
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
                                        value: 398,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: LetVariable {
                                stmt: 5,
                            },
                        },
                        caching_class: Variable,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 393,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            Literal::F32(
                                TermF32Literal {
                                    value: OrderedFloat(
                                        5.5,
                                    ),
                                    text: "5.5f32",
                                },
                            ),
                        ),
                        arguments: [],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
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
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 393,
                                },
                            ),
                        ),
                        opn: ValOpn::Binary(
                            Comparison(
                                Less,
                            ),
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 400,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 402,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
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
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 404,
                                },
                            ),
                        ),
                        opn: ValOpn::TypeVariant(
                            TypeVariantPath(
                                ItemPathId(
                                    Id {
                                        value: 252,
                                    },
                                ),
                            ),
                        ),
                        arguments: [],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
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
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorVal {
                                    path: FugitivePath(`mnist_classifier::major::major_concave_components`, `Val`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: KiReprSource::ValItem(
                            FugitivePath(`mnist_classifier::major::major_concave_components`, `Val`),
                        ),
                        caching_class: ValItem,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 407,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
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
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
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
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 407,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorRitchieEager {
                                    path: FugitivePath(`mnist_classifier::fermi::fermi_match`, `Ritchie(
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
                                        value: 408,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
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
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 407,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorRitchieEager {
                                    path: FugitivePath(`mnist_classifier::fermi::fermi_match`, `Ritchie(
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
                                        value: 408,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
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
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 407,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::fermi::FermiMatchResult(0)::norm`, `MemoizedField`),
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
                                        value: 410,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
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
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 407,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorRitchieEager {
                                    path: FugitivePath(`mnist_classifier::fermi::fermi_match`, `Ritchie(
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
                                        value: 408,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
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
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 407,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::fermi::FermiMatchResult(0)::rel_norm`, `MemoizedField`),
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
                                        value: 410,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
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
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 407,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorRitchieEager {
                                    path: FugitivePath(`mnist_classifier::fermi::fermi_match`, `Ritchie(
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
                                        value: 408,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
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
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 407,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::fermi::FermiMatchResult(0)::angle_change_norm`, `MemoizedField`),
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
                                        value: 410,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
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
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 407,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            Literal::I32(
                                5,
                            ),
                        ),
                        arguments: [],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 41,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 407,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorRitchieLazy {
                                    path: FugitivePath(`malamute::narrow_down`, `Ritchie(
                                        Gn,
                                    )`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [
                                            (
                                                Type(
                                                    Type {
                                                        attrs: HirTemplateSvarAttrs {
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
                                            value: 411,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 412,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 413,
                                        },
                                    ),
                                ],
                            ),
                            Keyed(
                                Some(
                                    KiRepr(
                                        Id {
                                            value: 414,
                                        },
                                    ),
                                ),
                            ),
                            RuntimeConstants(
                                [
                                    ValRuntimeConstant(
                                        Id {
                                            value: 3,
                                        },
                                    ),
                                ],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
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
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 407,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::UnveilAssocFn {
                                    path: TraitForTypeItemPath(
                                        ItemPathId(
                                            Id {
                                                value: 349,
                                            },
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
                                        value: 415,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
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
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 407,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorRitchieEager {
                                    path: FugitivePath(`mnist_classifier::fermi::fermi_match`, `Ritchie(
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
                                        value: 408,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
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
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 416,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::fermi::FermiMatchResult(0)::norm`, `MemoizedField`),
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
                                        value: 410,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 45,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 416,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            Literal::F32(
                                TermF32Literal {
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
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
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
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 416,
                                },
                            ),
                        ),
                        opn: ValOpn::Binary(
                            Comparison(
                                Less,
                            ),
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 418,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 419,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
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
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorVal {
                                    path: FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: KiReprSource::ValItem(
                            FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                        ),
                        caching_class: ValItem,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 421,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::connected_component::ConnectedComponent(0)::eff_holes`, `MemoizedField`),
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
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 49,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 421,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::StructField {
                                    self_ty: LinTypePathLeading {
                                        ty_path: TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
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
                                        value: 423,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 50,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 421,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            Literal::USize(
                                TermUSizeLiteral {
                                    value: 1,
                                },
                            ),
                        ),
                        arguments: [],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
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
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 421,
                                },
                            ),
                        ),
                        opn: ValOpn::Index,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 424,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 425,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
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
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorVal {
                                    path: FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: KiReprSource::ValItem(
                            FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                        ),
                        caching_class: ValItem,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 428,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::connected_component::ConnectedComponent(0)::eff_holes`, `MemoizedField`),
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
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 54,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 428,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::StructField {
                                    self_ty: LinTypePathLeading {
                                        ty_path: TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
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
                                        value: 430,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 55,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 428,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            Literal::USize(
                                TermUSizeLiteral {
                                    value: 0,
                                },
                            ),
                        ),
                        arguments: [],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
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
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 428,
                                },
                            ),
                        ),
                        opn: ValOpn::Index,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 431,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 432,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
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
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorVal {
                                    path: FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: KiReprSource::ValItem(
                            FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                        ),
                        caching_class: ValItem,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 435,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::connected_component::ConnectedComponent(0)::eff_holes`, `MemoizedField`),
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
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
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
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 435,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::StructField {
                                    self_ty: LinTypePathLeading {
                                        ty_path: TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
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
                                        value: 436,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
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
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 435,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            Literal::USize(
                                TermUSizeLiteral {
                                    value: 0,
                                },
                            ),
                        ),
                        arguments: [],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
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
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 435,
                                },
                            ),
                        ),
                        opn: ValOpn::Index,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 437,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 438,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 62,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 435,
                                },
                            ),
                        ),
                        opn: ValOpn::Index,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 437,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 438,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: LetVariable {
                                stmt: 14,
                            },
                        },
                        caching_class: Variable,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 435,
                                },
                            ),
                        ),
                        opn: ValOpn::Unwrap,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 440,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
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
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 435,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::raw_contour::RawContour(0)::bounding_box`, `MemoizedField`),
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
                                        value: 441,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 65,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 435,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MethodRitchie {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::geom2d::BoundingBox(0)::ymax`, `MethodRitchie(
                                            Fn,
                                        )`),
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
                                        value: 442,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
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
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 435,
                                },
                            ),
                        ),
                        opn: ValOpn::Index,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 437,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 438,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: LetVariable {
                                stmt: 14,
                            },
                        },
                        caching_class: Variable,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 435,
                                },
                            ),
                        ),
                        opn: ValOpn::Unwrap,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 440,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
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
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 435,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::raw_contour::RawContour(0)::bounding_box`, `MemoizedField`),
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
                                        value: 444,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
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
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 435,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MethodRitchie {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::geom2d::BoundingBox(0)::ymin`, `MethodRitchie(
                                            Fn,
                                        )`),
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
                                        value: 445,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
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
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 435,
                                },
                            ),
                        ),
                        opn: ValOpn::Binary(
                            Closed(
                                Sub,
                            ),
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 443,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 446,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 71,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorVal {
                                    path: FugitivePath(`mnist_classifier::major::major_line_segment_sketch`, `Val`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: KiReprSource::ValItem(
                            FugitivePath(`mnist_classifier::major::major_line_segment_sketch`, `Val`),
                        ),
                        caching_class: ValItem,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 435,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::line_segment_sketch::LineSegmentSketch(0)::bounding_box`, `MemoizedField`),
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
                                        value: 303,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
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
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 435,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MethodRitchie {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::geom2d::BoundingBox(0)::ymax`, `MethodRitchie(
                                            Fn,
                                        )`),
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
                                        value: 449,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
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
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorVal {
                                    path: FugitivePath(`mnist_classifier::major::major_line_segment_sketch`, `Val`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: KiReprSource::ValItem(
                            FugitivePath(`mnist_classifier::major::major_line_segment_sketch`, `Val`),
                        ),
                        caching_class: ValItem,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 435,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::line_segment_sketch::LineSegmentSketch(0)::bounding_box`, `MemoizedField`),
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
                                        value: 303,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
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
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 435,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MethodRitchie {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::geom2d::BoundingBox(0)::ymin`, `MethodRitchie(
                                            Fn,
                                        )`),
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
                                        value: 451,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
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
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 435,
                                },
                            ),
                        ),
                        opn: ValOpn::Binary(
                            Closed(
                                Sub,
                            ),
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 450,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 452,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
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
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 435,
                                },
                            ),
                        ),
                        opn: ValOpn::Binary(
                            Closed(
                                Sub,
                            ),
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 443,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 446,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: LetVariable {
                                stmt: 15,
                            },
                        },
                        caching_class: Variable,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 435,
                                },
                            ),
                        ),
                        opn: ValOpn::Binary(
                            Closed(
                                Sub,
                            ),
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 450,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 452,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: LetVariable {
                                stmt: 16,
                            },
                        },
                        caching_class: Variable,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 435,
                                },
                            ),
                        ),
                        opn: ValOpn::Binary(
                            Closed(
                                Div,
                            ),
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 448,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 454,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
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
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 435,
                                },
                            ),
                        ),
                        opn: ValOpn::Binary(
                            Closed(
                                Div,
                            ),
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 448,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 454,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
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
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 435,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            Literal::F32(
                                TermF32Literal {
                                    value: OrderedFloat(
                                        0.4,
                                    ),
                                    text: "0.4f32",
                                },
                            ),
                        ),
                        arguments: [],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
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
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 435,
                                },
                            ),
                        ),
                        opn: ValOpn::Binary(
                            Comparison(
                                Greater,
                            ),
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 456,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 458,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 84,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 407,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorRitchieEager {
                                    path: FugitivePath(`mnist_classifier::fermi::fermi_match`, `Ritchie(
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
                                        value: 408,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
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
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 460,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::fermi::FermiMatchResult(0)::norm`, `MemoizedField`),
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
                                        value: 410,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
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
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 460,
                                },
                            ),
                        ),
                        opn: ValOpn::TypeVariant(
                            TypeVariantPath(
                                ItemPathId(
                                    Id {
                                        value: 252,
                                    },
                                ),
                            ),
                        ),
                        arguments: [],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 87,
                            },
                        },
                        caching_class: Expr,
                    },
                ],
                hir_lazy_stmt_ki_repr_map: [
                    KiRepr {
                        val_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 375,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 380,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 378,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
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
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 381,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 386,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 382,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Stmt {
                                stmt: 3,
                            },
                        },
                        caching_class: Stmt,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 387,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 392,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 388,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
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
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 393,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 403,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 401,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
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
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 404,
                                },
                            ),
                        ),
                        opn: ValOpn::Return,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 405,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
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
                        val_domain_repr: Omni,
                        opn: ValOpn::Branches,
                        arguments: [
                            Branch {
                                condition: Some(
                                    KiRepr(
                                        Id {
                                            value: 375,
                                        },
                                    ),
                                ),
                                stmts: [
                                    KiRepr(
                                        Id {
                                            value: 381,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 387,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 393,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 404,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 406,
                                        },
                                    ),
                                ],
                            },
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
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
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 416,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 420,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 417,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
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
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 421,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 427,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 422,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Stmt {
                                stmt: 12,
                            },
                        },
                        caching_class: Stmt,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 428,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 434,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 429,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Stmt {
                                stmt: 13,
                            },
                        },
                        caching_class: Stmt,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 435,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 459,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 457,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Stmt {
                                stmt: 18,
                            },
                        },
                        caching_class: Stmt,
                    },
                ],
                root_hir_lazy_stmt_ki_reprs: [
                    KiRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Branches,
                        arguments: [
                            Branch {
                                condition: Some(
                                    KiRepr(
                                        Id {
                                            value: 375,
                                        },
                                    ),
                                ),
                                stmts: [
                                    KiRepr(
                                        Id {
                                            value: 381,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 387,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 393,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 404,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 406,
                                        },
                                    ),
                                ],
                            },
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
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
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 407,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::UnveilAssocFn {
                                    path: TraitForTypeItemPath(
                                        ItemPathId(
                                            Id {
                                                value: 349,
                                            },
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
                                        value: 415,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
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
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 416,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 420,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 417,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
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
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 421,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 427,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 422,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Stmt {
                                stmt: 12,
                            },
                        },
                        caching_class: Stmt,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 428,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 434,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 429,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Stmt {
                                stmt: 13,
                            },
                        },
                        caching_class: Stmt,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 435,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 459,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 457,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Stmt {
                                stmt: 18,
                            },
                        },
                        caching_class: Stmt,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 460,
                                },
                            ),
                        ),
                        opn: ValOpn::TypeVariant(
                            TypeVariantPath(
                                ItemPathId(
                                    Id {
                                        value: 252,
                                    },
                                ),
                            ),
                        ),
                        arguments: [],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 87,
                            },
                        },
                        caching_class: Expr,
                    },
                ],
            },
        ),
    ),
]