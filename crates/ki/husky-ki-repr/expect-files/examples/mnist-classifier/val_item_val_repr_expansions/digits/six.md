[
    (
        FugitivePath(`mnist_classifier::digits::six::six_match`, `Val`),
        None,
    ),
    (
        FugitivePath(`mnist_classifier::digits::six::six_match_refined1`, `Val`),
        None,
    ),
    (
        FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
        Some(
            ValReprExpansion {
                hir_lazy_variable_val_repr_map: ArenaMap {
                    data: [
                        Some(
                            KiRepr(
                                Id {
                                    value: 248,
                                },
                            ),
                        ),
                        Some(
                            KiRepr(
                                Id {
                                    value: 253,
                                },
                            ),
                        ),
                        Some(
                            KiRepr(
                                Id {
                                    value: 257,
                                },
                            ),
                        ),
                        Some(
                            KiRepr(
                                Id {
                                    value: 270,
                                },
                            ),
                        ),
                        Some(
                            KiRepr(
                                Id {
                                    value: 273,
                                },
                            ),
                        ),
                        Some(
                            KiRepr(
                                Id {
                                    value: 275,
                                },
                            ),
                        ),
                        Some(
                            KiRepr(
                                Id {
                                    value: 279,
                                },
                            ),
                        ),
                        Some(
                            KiRepr(
                                Id {
                                    value: 281,
                                },
                            ),
                        ),
                        Some(
                            KiRepr(
                                Id {
                                    value: 308,
                                },
                            ),
                        ),
                    ],
                },
                hir_lazy_expr_val_repr_map: [
                    KiRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorVal {
                                    path: FugitivePath(`mnist_classifier::digits::six::six_match`, `Val`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: ValReprSource::ValItem(
                            FugitivePath(`mnist_classifier::digits::six::six_match`, `Val`),
                        ),
                        caching_class: ValItem,
                    },
                    KiRepr {
                        val_domain_repr: Omni,
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
                                        value: 242,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
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
                        opn: ValOpn::Literal(
                            Literal::USize(
                                TermUSizeLiteral {
                                    value: 0,
                                },
                            ),
                        ),
                        arguments: [],
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
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
                        opn: ValOpn::Index,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 245,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 246,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
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
                        opn: ValOpn::Index,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 245,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 246,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: LetVariable {
                                stmt: 20,
                            },
                        },
                        caching_class: Variable,
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
                        source: ValReprSource::ValItem(
                            FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                        ),
                        caching_class: ValItem,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 251,
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
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
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
                        source: ValReprSource::ValItem(
                            FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                        ),
                        caching_class: ValItem,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 251,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::connected_component::ConnectedComponent(0)::lower_mass`, `MemoizedField`),
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
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
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
                        source: ValReprSource::ValItem(
                            FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                        ),
                        caching_class: ValItem,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 251,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::connected_component::ConnectedComponent(0)::upper_mass`, `MemoizedField`),
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
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 11,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 251,
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
                                        value: 254,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 255,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
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
                                    value: 251,
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
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: LetVariable {
                                stmt: 22,
                            },
                        },
                        caching_class: Variable,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 251,
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
                                        value: 253,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
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
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 251,
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
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 15,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 251,
                                },
                            ),
                        ),
                        opn: ValOpn::Index,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 258,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 259,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
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
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorVal {
                                    path: FugitivePath(`mnist_classifier::digits::six::six_match`, `Val`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: ValReprSource::ValItem(
                            FugitivePath(`mnist_classifier::digits::six::six_match`, `Val`),
                        ),
                        caching_class: ValItem,
                    },
                    KiRepr {
                        val_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 261,
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
                                        value: 242,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
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
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorVal {
                                    path: FugitivePath(`mnist_classifier::digits::six::six_match_refined1`, `Val`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: ValReprSource::ValItem(
                            FugitivePath(`mnist_classifier::digits::six::six_match_refined1`, `Val`),
                        ),
                        caching_class: ValItem,
                    },
                    KiRepr {
                        val_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 261,
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
                                        value: 243,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
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
                        val_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 261,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            Literal::I32(
                                5,
                            ),
                        ),
                        arguments: [],
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
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
                        val_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 261,
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
                                            value: 262,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 263,
                                        },
                                    ),
                                ],
                            ),
                            Keyed(
                                Some(
                                    KiRepr(
                                        Id {
                                            value: 264,
                                        },
                                    ),
                                ),
                            ),
                            RuntimeConstants(
                                [
                                    ValRuntimeConstant(
                                        Id {
                                            value: 2,
                                        },
                                    ),
                                ],
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
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
                        val_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 261,
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
                                        value: 265,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
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
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorVal {
                                    path: FugitivePath(`mnist_classifier::digits::six::six_match_refined1`, `Val`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: ValReprSource::ValItem(
                            FugitivePath(`mnist_classifier::digits::six::six_match_refined1`, `Val`),
                        ),
                        caching_class: ValItem,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 266,
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
                                        value: 243,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
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
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 266,
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
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
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
                                    value: 266,
                                },
                            ),
                        ),
                        opn: ValOpn::Index,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 267,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 268,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
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
                                    value: 266,
                                },
                            ),
                        ),
                        opn: ValOpn::Index,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 267,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 268,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: LetVariable {
                                stmt: 7,
                            },
                        },
                        caching_class: Variable,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 266,
                                },
                            ),
                        ),
                        opn: ValOpn::Unwrap,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 270,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 30,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 266,
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
                                        value: 271,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
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
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 266,
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
                                        value: 271,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: LetVariable {
                                stmt: 8,
                            },
                        },
                        caching_class: Variable,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 266,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
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
                                        value: 273,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
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
                        val_domain_repr: Omni,
                        opn: ValOpn::Index,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 245,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 246,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: LetVariable {
                                stmt: 20,
                            },
                        },
                        caching_class: Variable,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 266,
                                },
                            ),
                        ),
                        opn: ValOpn::Unwrap,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 248,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
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
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 266,
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
                                        value: 276,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
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
                                    value: 266,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
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
                                        value: 277,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 37,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorVal {
                                    path: FugitivePath(`mnist_classifier::digits::six::six_match_refined1`, `Val`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: ValReprSource::ValItem(
                            FugitivePath(`mnist_classifier::digits::six::six_match_refined1`, `Val`),
                        ),
                        caching_class: ValItem,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 266,
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
                                                    value: 277,
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
                                        value: 243,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
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
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorVal {
                                    path: FugitivePath(`mnist_classifier::digits::six::six_match_refined1`, `Val`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: ValReprSource::ValItem(
                            FugitivePath(`mnist_classifier::digits::six::six_match_refined1`, `Val`),
                        ),
                        caching_class: ValItem,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 266,
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
                                        value: 243,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
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
                                    value: 266,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            Literal::F32(
                                TermF32Literal {
                                    value: OrderedFloat(
                                        1.8,
                                    ),
                                    text: "1.8f32",
                                },
                            ),
                        ),
                        arguments: [],
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
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
                                    value: 266,
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
                                        value: 283,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 284,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
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
                                    value: 266,
                                },
                            ),
                        ),
                        opn: ValOpn::Index,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 267,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 268,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: LetVariable {
                                stmt: 7,
                            },
                        },
                        caching_class: Variable,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 266,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
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
                                        value: 273,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
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
                        val_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 287,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            Literal::F32(
                                TermF32Literal {
                                    value: OrderedFloat(
                                        2.5,
                                    ),
                                    text: "2.5f32",
                                },
                            ),
                        ),
                        arguments: [],
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
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
                        val_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 287,
                                },
                            ),
                        ),
                        opn: ValOpn::Prefix(
                            Minus,
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 289,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
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
                        val_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 287,
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
                                        value: 275,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 290,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
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
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorVal {
                                    path: FugitivePath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Val`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: ValReprSource::ValItem(
                            FugitivePath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Val`),
                        ),
                        caching_class: ValItem,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 292,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            Literal::F32(
                                TermF32Literal {
                                    value: OrderedFloat(
                                        30.0,
                                    ),
                                    text: "30.0f32",
                                },
                            ),
                        ),
                        arguments: [],
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
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
                                    value: 292,
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
                                        value: 16,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 294,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
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
                                    value: 266,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
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
                                        value: 277,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
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
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 296,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            Literal::I32(
                                15,
                            ),
                        ),
                        arguments: [],
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
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
                                    value: 296,
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
                                            value: 279,
                                        },
                                    ),
                                ],
                            ),
                            Keyed(
                                Some(
                                    KiRepr(
                                        Id {
                                            value: 297,
                                        },
                                    ),
                                ),
                            ),
                            RuntimeConstants(
                                [
                                    ValRuntimeConstant(
                                        Id {
                                            value: 2,
                                        },
                                    ),
                                ],
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
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
                                    value: 296,
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
                                        value: 298,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
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
                                    value: 299,
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
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
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
                                    path: FugitivePath(`mnist_classifier::major::major_line_segment_sketch`, `Val`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: ValReprSource::ValItem(
                            FugitivePath(`mnist_classifier::major::major_line_segment_sketch`, `Val`),
                        ),
                        caching_class: ValItem,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 302,
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
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
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
                        val_domain_repr: Omni,
                        opn: ValOpn::Index,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 245,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 246,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: LetVariable {
                                stmt: 20,
                            },
                        },
                        caching_class: Variable,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 302,
                                },
                            ),
                        ),
                        opn: ValOpn::Unwrap,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 248,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
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
                                    value: 302,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MethodRitchie {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::end`, `MethodRitchie(
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
                                        value: 305,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
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
                                    value: 302,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MethodRitchie {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::geom2d::BoundingBox(0)::relative_point`, `MethodRitchie(
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
                                        value: 304,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 306,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
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
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 266,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
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
                                        value: 277,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
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
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorVal {
                                    path: FugitivePath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Val`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: ValReprSource::ValItem(
                            FugitivePath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Val`),
                        ),
                        caching_class: ValItem,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 251,
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
                                        value: 254,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 255,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: LetVariable {
                                stmt: 23,
                            },
                        },
                        caching_class: Variable,
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
                        source: ValReprSource::ValItem(
                            FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                        ),
                        caching_class: ValItem,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 302,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            Literal::I32(
                                6,
                            ),
                        ),
                        arguments: [],
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
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
                                    value: 302,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MethodRitchie {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::connected_component::ConnectedComponent(0)::top_k_row_span_sum`, `MethodRitchie(
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
                                        value: 14,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 309,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
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
                                    value: 302,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            Literal::I32(
                                15,
                            ),
                        ),
                        arguments: [],
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
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
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 302,
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
                                            value: 279,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 16,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 257,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 310,
                                        },
                                    ),
                                ],
                            ),
                            Keyed(
                                Some(
                                    KiRepr(
                                        Id {
                                            value: 311,
                                        },
                                    ),
                                ),
                            ),
                            RuntimeConstants(
                                [
                                    ValRuntimeConstant(
                                        Id {
                                            value: 2,
                                        },
                                    ),
                                ],
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
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
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 302,
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
                                        value: 312,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
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
                                    value: 302,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MethodRitchie {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::geom2d::BoundingBox(0)::relative_point`, `MethodRitchie(
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
                                        value: 304,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 306,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
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
                                    value: 313,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::StructField {
                                    self_ty: LinTypePathLeading {
                                        ty_path: TypePath(`mnist_classifier::geom2d::RelativePoint2d`, `Struct`),
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
                                        value: 308,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 75,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 313,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            Literal::F32(
                                TermF32Literal {
                                    value: OrderedFloat(
                                        0.7,
                                    ),
                                    text: "0.7f32",
                                },
                            ),
                        ),
                        arguments: [],
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
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
                                    value: 313,
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
                                        value: 314,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 315,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
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
                        val_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 316,
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
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
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
                                    value: 319,
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
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
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
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorVal {
                                    path: FugitivePath(`mnist_classifier::digits::six::six_match`, `Val`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: ValReprSource::ValItem(
                            FugitivePath(`mnist_classifier::digits::six::six_match`, `Val`),
                        ),
                        caching_class: ValItem,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 322,
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
                                        value: 242,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
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
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 322,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            Literal::I32(
                                5,
                            ),
                        ),
                        arguments: [],
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
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
                                    value: 322,
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
                                            value: 323,
                                        },
                                    ),
                                ],
                            ),
                            Keyed(
                                Some(
                                    KiRepr(
                                        Id {
                                            value: 324,
                                        },
                                    ),
                                ),
                            ),
                            RuntimeConstants(
                                [
                                    ValRuntimeConstant(
                                        Id {
                                            value: 2,
                                        },
                                    ),
                                ],
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
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
                                    value: 322,
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
                                        value: 325,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 85,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorVal {
                                    path: FugitivePath(`mnist_classifier::digits::six::six_match`, `Val`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: ValReprSource::ValItem(
                            FugitivePath(`mnist_classifier::digits::six::six_match`, `Val`),
                        ),
                        caching_class: ValItem,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 326,
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
                                        value: 242,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
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
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 326,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            Literal::F32(
                                TermF32Literal {
                                    value: OrderedFloat(
                                        1.8,
                                    ),
                                    text: "1.8f32",
                                },
                            ),
                        ),
                        arguments: [],
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 88,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 326,
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
                                        value: 327,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 328,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
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
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorVal {
                                    path: FugitivePath(`mnist_classifier::digits::six::six_match`, `Val`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: ValReprSource::ValItem(
                            FugitivePath(`mnist_classifier::digits::six::six_match`, `Val`),
                        ),
                        caching_class: ValItem,
                    },
                    KiRepr {
                        val_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 329,
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
                                        value: 242,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
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
                        val_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 329,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            Literal::F32(
                                TermF32Literal {
                                    value: OrderedFloat(
                                        1.8,
                                    ),
                                    text: "1.8f32",
                                },
                            ),
                        ),
                        arguments: [],
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 92,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 329,
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
                                        value: 331,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 332,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
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
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 251,
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
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: LetVariable {
                                stmt: 22,
                            },
                        },
                        caching_class: Variable,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 335,
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
                                        value: 253,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
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
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 335,
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
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
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
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 335,
                                },
                            ),
                        ),
                        opn: ValOpn::Index,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 337,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 338,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 97,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 335,
                                },
                            ),
                        ),
                        opn: ValOpn::Unwrap,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 339,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 98,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 335,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::raw_contour::RawContour(0)::relative_bounding_box`, `MemoizedField`),
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
                                        value: 340,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
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
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 335,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MethodRitchie {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::geom2d::RelativeBoundingBox(0)::ymax`, `MethodRitchie(
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
                                        value: 341,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
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
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 335,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            Literal::F32(
                                TermF32Literal {
                                    value: OrderedFloat(
                                        0.75,
                                    ),
                                    text: "0.75f32",
                                },
                            ),
                        ),
                        arguments: [],
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
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
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 335,
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
                                        value: 342,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 343,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
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
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 251,
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
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: LetVariable {
                                stmt: 22,
                            },
                        },
                        caching_class: Variable,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 345,
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
                                        value: 253,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 104,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 345,
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
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
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
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 345,
                                },
                            ),
                        ),
                        opn: ValOpn::Index,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 346,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 347,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
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
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 251,
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
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: LetVariable {
                                stmt: 22,
                            },
                        },
                        caching_class: Variable,
                    },
                    KiRepr {
                        val_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 349,
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
                                        value: 253,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
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
                        val_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 349,
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
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
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
                        val_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 349,
                                },
                            ),
                        ),
                        opn: ValOpn::Index,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 351,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 352,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
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
                        val_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 349,
                                },
                            ),
                        ),
                        opn: ValOpn::Unwrap,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 353,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
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
                        val_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 349,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::raw_contour::RawContour(0)::relative_bounding_box`, `MemoizedField`),
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
                                        value: 354,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
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
                        val_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 349,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MethodRitchie {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::geom2d::RelativeBoundingBox(0)::ymax`, `MethodRitchie(
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
                                        value: 355,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
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
                        val_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 349,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            Literal::F32(
                                TermF32Literal {
                                    value: OrderedFloat(
                                        0.75,
                                    ),
                                    text: "0.75f32",
                                },
                            ),
                        ),
                        arguments: [],
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 114,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 349,
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
                                        value: 356,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 357,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 115,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 251,
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
                                        value: 254,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 255,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: LetVariable {
                                stmt: 23,
                            },
                        },
                        caching_class: Variable,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 360,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            Literal::F32(
                                TermF32Literal {
                                    value: OrderedFloat(
                                        15.0,
                                    ),
                                    text: "15.0f32",
                                },
                            ),
                        ),
                        arguments: [],
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
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
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 360,
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
                                        value: 257,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 362,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
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
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 364,
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
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 119,
                            },
                        },
                        caching_class: Expr,
                    },
                ],
                hir_lazy_stmt_val_repr_map: [
                    KiRepr {
                        val_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 287,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 291,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 288,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Stmt {
                                stmt: 1,
                            },
                        },
                        caching_class: Stmt,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 292,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 295,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 293,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
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
                                    value: 299,
                                },
                            ),
                        ),
                        opn: ValOpn::Return,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 300,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
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
                        val_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 316,
                                },
                            ),
                        ),
                        opn: ValOpn::Return,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 317,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Stmt {
                                stmt: 5,
                            },
                        },
                        caching_class: Stmt,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 266,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 285,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 282,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
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
                                    value: 286,
                                },
                            ),
                        ),
                        opn: ValOpn::Branches,
                        arguments: [
                            Branch {
                                condition: Some(
                                    KiRepr(
                                        Id {
                                            value: 287,
                                        },
                                    ),
                                ),
                                stmts: [
                                    KiRepr(
                                        Id {
                                            value: 292,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 296,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 299,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 301,
                                        },
                                    ),
                                ],
                            },
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
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
                                    value: 313,
                                },
                            ),
                        ),
                        opn: ValOpn::Branches,
                        arguments: [
                            Branch {
                                condition: Some(
                                    KiRepr(
                                        Id {
                                            value: 316,
                                        },
                                    ),
                                ),
                                stmts: [
                                    KiRepr(
                                        Id {
                                            value: 318,
                                        },
                                    ),
                                ],
                            },
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
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
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 319,
                                },
                            ),
                        ),
                        opn: ValOpn::Return,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 320,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Stmt {
                                stmt: 17,
                            },
                        },
                        caching_class: Stmt,
                    },
                    KiRepr {
                        val_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 329,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 333,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 330,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
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
                        val_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 349,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 358,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 350,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Stmt {
                                stmt: 19,
                            },
                        },
                        caching_class: Stmt,
                    },
                    KiRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Require,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 250,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 249,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Stmt {
                                stmt: 21,
                            },
                        },
                        caching_class: Stmt,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 251,
                                },
                            ),
                        ),
                        opn: ValOpn::Branches,
                        arguments: [
                            Branch {
                                condition: Some(
                                    KiRepr(
                                        Id {
                                            value: 261,
                                        },
                                    ),
                                ),
                                stmts: [
                                    KiRepr(
                                        Id {
                                            value: 266,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 286,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 302,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 313,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 319,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 321,
                                        },
                                    ),
                                ],
                            },
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Stmt {
                                stmt: 24,
                            },
                        },
                        caching_class: Stmt,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 326,
                                },
                            ),
                        ),
                        opn: ValOpn::Branches,
                        arguments: [
                            Branch {
                                condition: Some(
                                    KiRepr(
                                        Id {
                                            value: 329,
                                        },
                                    ),
                                ),
                                stmts: [
                                    KiRepr(
                                        Id {
                                            value: 334,
                                        },
                                    ),
                                ],
                            },
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
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
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 335,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 344,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 336,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
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
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 345,
                                },
                            ),
                        ),
                        opn: ValOpn::Branches,
                        arguments: [
                            Branch {
                                condition: Some(
                                    KiRepr(
                                        Id {
                                            value: 349,
                                        },
                                    ),
                                ),
                                stmts: [
                                    KiRepr(
                                        Id {
                                            value: 359,
                                        },
                                    ),
                                ],
                            },
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Stmt {
                                stmt: 28,
                            },
                        },
                        caching_class: Stmt,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 360,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 363,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 361,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Stmt {
                                stmt: 29,
                            },
                        },
                        caching_class: Stmt,
                    },
                ],
                root_hir_lazy_stmt_val_reprs: [
                    KiRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Require,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 250,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 249,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Stmt {
                                stmt: 21,
                            },
                        },
                        caching_class: Stmt,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 251,
                                },
                            ),
                        ),
                        opn: ValOpn::Branches,
                        arguments: [
                            Branch {
                                condition: Some(
                                    KiRepr(
                                        Id {
                                            value: 261,
                                        },
                                    ),
                                ),
                                stmts: [
                                    KiRepr(
                                        Id {
                                            value: 266,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 286,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 302,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 313,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 319,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 321,
                                        },
                                    ),
                                ],
                            },
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Stmt {
                                stmt: 24,
                            },
                        },
                        caching_class: Stmt,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 322,
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
                                        value: 325,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 85,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 326,
                                },
                            ),
                        ),
                        opn: ValOpn::Branches,
                        arguments: [
                            Branch {
                                condition: Some(
                                    KiRepr(
                                        Id {
                                            value: 329,
                                        },
                                    ),
                                ),
                                stmts: [
                                    KiRepr(
                                        Id {
                                            value: 334,
                                        },
                                    ),
                                ],
                            },
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
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
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 335,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 344,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 336,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
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
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 345,
                                },
                            ),
                        ),
                        opn: ValOpn::Branches,
                        arguments: [
                            Branch {
                                condition: Some(
                                    KiRepr(
                                        Id {
                                            value: 349,
                                        },
                                    ),
                                ),
                                stmts: [
                                    KiRepr(
                                        Id {
                                            value: 359,
                                        },
                                    ),
                                ],
                            },
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Stmt {
                                stmt: 28,
                            },
                        },
                        caching_class: Stmt,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 360,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 363,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 361,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Stmt {
                                stmt: 29,
                            },
                        },
                        caching_class: Stmt,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 364,
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
                        source: ValReprSource::Expansion {
                            parent_val_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 119,
                            },
                        },
                        caching_class: Expr,
                    },
                ],
            },
        ),
    ),
]