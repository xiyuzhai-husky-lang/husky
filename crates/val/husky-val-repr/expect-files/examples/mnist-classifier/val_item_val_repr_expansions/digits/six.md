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
                            ValRepr(
                                Id {
                                    value: 248,
                                },
                            ),
                        ),
                        Some(
                            ValRepr(
                                Id {
                                    value: 253,
                                },
                            ),
                        ),
                        Some(
                            ValRepr(
                                Id {
                                    value: 257,
                                },
                            ),
                        ),
                        Some(
                            ValRepr(
                                Id {
                                    value: 270,
                                },
                            ),
                        ),
                        Some(
                            ValRepr(
                                Id {
                                    value: 273,
                                },
                            ),
                        ),
                        Some(
                            ValRepr(
                                Id {
                                    value: 275,
                                },
                            ),
                        ),
                        Some(
                            ValRepr(
                                Id {
                                    value: 279,
                                },
                            ),
                        ),
                        Some(
                            ValRepr(
                                Id {
                                    value: 281,
                                },
                            ),
                        ),
                        Some(
                            ValRepr(
                                Id {
                                    value: 308,
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
                                    path: FugitivePath(`mnist_classifier::digits::six::six_match`, `Val`),
                                    instantiation: LinkageInstantiation {
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
                    ValRepr {
                        val_domain_repr: Omni,
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
                                        value: 242,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Literal(
                            TermLiteral::USize(
                                TermUSizeLiteral {
                                    value: 0,
                                },
                            ),
                        ),
                        arguments: [],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Index,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 245,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 246,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Index,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 245,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 246,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
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
                        source: ValReprSource::ValItem(
                            FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                        ),
                        caching_class: ValItem,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 251,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssociatedItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::connected_component::ConnectedComponent(0)::eff_holes`, `MemoizedField`),
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
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
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
                        source: ValReprSource::ValItem(
                            FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                        ),
                        caching_class: ValItem,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 251,
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
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
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
                        source: ValReprSource::ValItem(
                            FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                        ),
                        caching_class: ValItem,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 251,
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
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
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
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 254,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 255,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 251,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssociatedItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::connected_component::ConnectedComponent(0)::eff_holes`, `MemoizedField`),
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
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 251,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::StructField {
                                    self_ty: LinkageTypePathLeading {
                                        ty_path: TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
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
                                        value: 253,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 251,
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
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 251,
                                },
                            ),
                        ),
                        opn: ValOpn::Index,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 258,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 259,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::ValItem {
                                    path: FugitivePath(`mnist_classifier::digits::six::six_match`, `Val`),
                                    instantiation: LinkageInstantiation {
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
                    ValRepr {
                        val_domain_repr: ConditionSatisfied(
                            ValRepr(
                                Id {
                                    value: 261,
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
                                        value: 242,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::ValItem {
                                    path: FugitivePath(`mnist_classifier::digits::six::six_match_refined1`, `Val`),
                                    instantiation: LinkageInstantiation {
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
                    ValRepr {
                        val_domain_repr: ConditionSatisfied(
                            ValRepr(
                                Id {
                                    value: 261,
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
                                        value: 243,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: ConditionSatisfied(
                            ValRepr(
                                Id {
                                    value: 261,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            TermLiteral::I32(
                                5,
                            ),
                        ),
                        arguments: [],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: ConditionSatisfied(
                            ValRepr(
                                Id {
                                    value: 261,
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
                                            value: 262,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 263,
                                        },
                                    ),
                                ],
                            ),
                            Keyed(
                                Some(
                                    ValRepr(
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
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: ConditionSatisfied(
                            ValRepr(
                                Id {
                                    value: 261,
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
                                        value: 265,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::ValItem {
                                    path: FugitivePath(`mnist_classifier::digits::six::six_match_refined1`, `Val`),
                                    instantiation: LinkageInstantiation {
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
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 266,
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
                                        value: 243,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 266,
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
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 266,
                                },
                            ),
                        ),
                        opn: ValOpn::Index,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 267,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 268,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 266,
                                },
                            ),
                        ),
                        opn: ValOpn::Index,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 267,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 268,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 266,
                                },
                            ),
                        ),
                        opn: ValOpn::Unwrap,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 270,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 266,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MethodFn {
                                    path: AssociatedItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::displacement`, `MethodFn`),
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
                                        value: 271,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 266,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MethodFn {
                                    path: AssociatedItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::displacement`, `MethodFn`),
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
                                        value: 271,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 266,
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
                                        value: 273,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Index,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 245,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 246,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 266,
                                },
                            ),
                        ),
                        opn: ValOpn::Unwrap,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 248,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 266,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MethodFn {
                                    path: AssociatedItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::displacement`, `MethodFn`),
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
                                        value: 276,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 266,
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
                                        value: 277,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::ValItem {
                                    path: FugitivePath(`mnist_classifier::digits::six::six_match_refined1`, `Val`),
                                    instantiation: LinkageInstantiation {
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
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 266,
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
                                                    value: 425,
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
                                        value: 243,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::ValItem {
                                    path: FugitivePath(`mnist_classifier::digits::six::six_match_refined1`, `Val`),
                                    instantiation: LinkageInstantiation {
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
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 266,
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
                                        value: 243,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 266,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            TermLiteral::F32(
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
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
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
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 283,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 284,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 266,
                                },
                            ),
                        ),
                        opn: ValOpn::Index,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 267,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 268,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 266,
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
                                        value: 273,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: ConditionSatisfied(
                            ValRepr(
                                Id {
                                    value: 287,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            TermLiteral::F32(
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
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: ConditionSatisfied(
                            ValRepr(
                                Id {
                                    value: 287,
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
                                        value: 289,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: ConditionSatisfied(
                            ValRepr(
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
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 275,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 290,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
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
                        source: ValReprSource::ValItem(
                            FugitivePath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Val`),
                        ),
                        caching_class: ValItem,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 292,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            TermLiteral::F32(
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
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
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
                                        value: 294,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 266,
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
                                        value: 277,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 296,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            TermLiteral::I32(
                                15,
                            ),
                        ),
                        arguments: [],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 296,
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
                                            value: 279,
                                        },
                                    ),
                                ],
                            ),
                            Keyed(
                                Some(
                                    ValRepr(
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
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 296,
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
                                        value: 298,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 299,
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
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::ValItem {
                                    path: FugitivePath(`mnist_classifier::major::major_line_segment_sketch`, `Val`),
                                    instantiation: LinkageInstantiation {
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
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 302,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssociatedItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::line_segment_sketch::LineSegmentSketch(0)::bounding_box`, `MemoizedField`),
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
                                        value: 303,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Index,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 245,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 246,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 302,
                                },
                            ),
                        ),
                        opn: ValOpn::Unwrap,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 248,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 302,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MethodFn {
                                    path: AssociatedItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::end`, `MethodFn`),
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
                                        value: 305,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 302,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MethodFn {
                                    path: AssociatedItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::geom2d::BoundingBox(0)::relative_point`, `MethodFn`),
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
                                        value: 304,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
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
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 266,
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
                                        value: 277,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
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
                        source: ValReprSource::ValItem(
                            FugitivePath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Val`),
                        ),
                        caching_class: ValItem,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
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
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 254,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 255,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
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
                        source: ValReprSource::ValItem(
                            FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                        ),
                        caching_class: ValItem,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 302,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            TermLiteral::I32(
                                6,
                            ),
                        ),
                        arguments: [],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 302,
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
                                        value: 309,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 302,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            TermLiteral::I32(
                                15,
                            ),
                        ),
                        arguments: [],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 302,
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
                                            value: 279,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 16,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 257,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 310,
                                        },
                                    ),
                                ],
                            ),
                            Keyed(
                                Some(
                                    ValRepr(
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
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 302,
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
                                        value: 312,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 302,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MethodFn {
                                    path: AssociatedItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::geom2d::BoundingBox(0)::relative_point`, `MethodFn`),
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
                                        value: 304,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
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
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 313,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::StructField {
                                    self_ty: LinkageTypePathLeading {
                                        ty_path: TypePath(`mnist_classifier::geom2d::RelativePoint2d`, `Struct`),
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
                                        value: 308,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 313,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            TermLiteral::F32(
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
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
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
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 314,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 315,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: ConditionSatisfied(
                            ValRepr(
                                Id {
                                    value: 316,
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
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 319,
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
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::ValItem {
                                    path: FugitivePath(`mnist_classifier::digits::six::six_match`, `Val`),
                                    instantiation: LinkageInstantiation {
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
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 322,
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
                                        value: 242,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 322,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            TermLiteral::I32(
                                5,
                            ),
                        ),
                        arguments: [],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 322,
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
                                            value: 323,
                                        },
                                    ),
                                ],
                            ),
                            Keyed(
                                Some(
                                    ValRepr(
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
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 322,
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
                                        value: 325,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::ValItem {
                                    path: FugitivePath(`mnist_classifier::digits::six::six_match`, `Val`),
                                    instantiation: LinkageInstantiation {
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
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 326,
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
                                        value: 242,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 326,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            TermLiteral::F32(
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
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
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
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 327,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 328,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::ValItem {
                                    path: FugitivePath(`mnist_classifier::digits::six::six_match`, `Val`),
                                    instantiation: LinkageInstantiation {
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
                    ValRepr {
                        val_domain_repr: ConditionSatisfied(
                            ValRepr(
                                Id {
                                    value: 329,
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
                                        value: 242,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: ConditionSatisfied(
                            ValRepr(
                                Id {
                                    value: 329,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            TermLiteral::F32(
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
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: ConditionSatisfied(
                            ValRepr(
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
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 331,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 332,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 251,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssociatedItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::connected_component::ConnectedComponent(0)::eff_holes`, `MemoizedField`),
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
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 335,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::StructField {
                                    self_ty: LinkageTypePathLeading {
                                        ty_path: TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
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
                                        value: 253,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 335,
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
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 335,
                                },
                            ),
                        ),
                        opn: ValOpn::Index,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 337,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 338,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 335,
                                },
                            ),
                        ),
                        opn: ValOpn::Unwrap,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 339,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 335,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssociatedItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::raw_contour::RawContour(0)::relative_bounding_box`, `MemoizedField`),
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
                                        value: 340,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 335,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MethodFn {
                                    path: AssociatedItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::geom2d::RelativeBoundingBox(0)::ymax`, `MethodFn`),
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
                                        value: 341,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 335,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            TermLiteral::F32(
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
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
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
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 342,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 343,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 251,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssociatedItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::connected_component::ConnectedComponent(0)::eff_holes`, `MemoizedField`),
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
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 345,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::StructField {
                                    self_ty: LinkageTypePathLeading {
                                        ty_path: TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
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
                                        value: 253,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 345,
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
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 345,
                                },
                            ),
                        ),
                        opn: ValOpn::Index,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 346,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 347,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 251,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssociatedItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::connected_component::ConnectedComponent(0)::eff_holes`, `MemoizedField`),
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
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: ConditionSatisfied(
                            ValRepr(
                                Id {
                                    value: 349,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::StructField {
                                    self_ty: LinkageTypePathLeading {
                                        ty_path: TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
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
                                        value: 253,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: ConditionSatisfied(
                            ValRepr(
                                Id {
                                    value: 349,
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
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: ConditionSatisfied(
                            ValRepr(
                                Id {
                                    value: 349,
                                },
                            ),
                        ),
                        opn: ValOpn::Index,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 351,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 352,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: ConditionSatisfied(
                            ValRepr(
                                Id {
                                    value: 349,
                                },
                            ),
                        ),
                        opn: ValOpn::Unwrap,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 353,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: ConditionSatisfied(
                            ValRepr(
                                Id {
                                    value: 349,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssociatedItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::raw_contour::RawContour(0)::relative_bounding_box`, `MemoizedField`),
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
                                        value: 354,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: ConditionSatisfied(
                            ValRepr(
                                Id {
                                    value: 349,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MethodFn {
                                    path: AssociatedItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::geom2d::RelativeBoundingBox(0)::ymax`, `MethodFn`),
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
                                        value: 355,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: ConditionSatisfied(
                            ValRepr(
                                Id {
                                    value: 349,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            TermLiteral::F32(
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
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: ConditionSatisfied(
                            ValRepr(
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
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 356,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 357,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
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
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 254,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 255,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 360,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            TermLiteral::F32(
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
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
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
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 257,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 362,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 364,
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
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: ConditionSatisfied(
                            ValRepr(
                                Id {
                                    value: 287,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 291,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 288,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 292,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 295,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 293,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 299,
                                },
                            ),
                        ),
                        opn: ValOpn::Return,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 300,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: ConditionSatisfied(
                            ValRepr(
                                Id {
                                    value: 316,
                                },
                            ),
                        ),
                        opn: ValOpn::Return,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 317,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 266,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 285,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 282,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 286,
                                },
                            ),
                        ),
                        opn: ValOpn::Branches,
                        arguments: [
                            Branch {
                                condition: Some(
                                    ValRepr(
                                        Id {
                                            value: 287,
                                        },
                                    ),
                                ),
                                stmts: [
                                    ValRepr(
                                        Id {
                                            value: 292,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 296,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 299,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 301,
                                        },
                                    ),
                                ],
                            },
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 313,
                                },
                            ),
                        ),
                        opn: ValOpn::Branches,
                        arguments: [
                            Branch {
                                condition: Some(
                                    ValRepr(
                                        Id {
                                            value: 316,
                                        },
                                    ),
                                ),
                                stmts: [
                                    ValRepr(
                                        Id {
                                            value: 318,
                                        },
                                    ),
                                ],
                            },
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 319,
                                },
                            ),
                        ),
                        opn: ValOpn::Return,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 320,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: ConditionSatisfied(
                            ValRepr(
                                Id {
                                    value: 329,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 333,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 330,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: ConditionSatisfied(
                            ValRepr(
                                Id {
                                    value: 349,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 358,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 350,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Require,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 250,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 249,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 251,
                                },
                            ),
                        ),
                        opn: ValOpn::Branches,
                        arguments: [
                            Branch {
                                condition: Some(
                                    ValRepr(
                                        Id {
                                            value: 261,
                                        },
                                    ),
                                ),
                                stmts: [
                                    ValRepr(
                                        Id {
                                            value: 266,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 286,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 302,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 313,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 319,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 321,
                                        },
                                    ),
                                ],
                            },
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 326,
                                },
                            ),
                        ),
                        opn: ValOpn::Branches,
                        arguments: [
                            Branch {
                                condition: Some(
                                    ValRepr(
                                        Id {
                                            value: 329,
                                        },
                                    ),
                                ),
                                stmts: [
                                    ValRepr(
                                        Id {
                                            value: 334,
                                        },
                                    ),
                                ],
                            },
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 335,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 344,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 336,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 345,
                                },
                            ),
                        ),
                        opn: ValOpn::Branches,
                        arguments: [
                            Branch {
                                condition: Some(
                                    ValRepr(
                                        Id {
                                            value: 349,
                                        },
                                    ),
                                ),
                                stmts: [
                                    ValRepr(
                                        Id {
                                            value: 359,
                                        },
                                    ),
                                ],
                            },
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 360,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 363,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 361,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Require,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 250,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 249,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 251,
                                },
                            ),
                        ),
                        opn: ValOpn::Branches,
                        arguments: [
                            Branch {
                                condition: Some(
                                    ValRepr(
                                        Id {
                                            value: 261,
                                        },
                                    ),
                                ),
                                stmts: [
                                    ValRepr(
                                        Id {
                                            value: 266,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 286,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 302,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 313,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 319,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 321,
                                        },
                                    ),
                                ],
                            },
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 322,
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
                                        value: 325,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 326,
                                },
                            ),
                        ),
                        opn: ValOpn::Branches,
                        arguments: [
                            Branch {
                                condition: Some(
                                    ValRepr(
                                        Id {
                                            value: 329,
                                        },
                                    ),
                                ),
                                stmts: [
                                    ValRepr(
                                        Id {
                                            value: 334,
                                        },
                                    ),
                                ],
                            },
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 335,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 344,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 336,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 345,
                                },
                            ),
                        ),
                        opn: ValOpn::Branches,
                        arguments: [
                            Branch {
                                condition: Some(
                                    ValRepr(
                                        Id {
                                            value: 349,
                                        },
                                    ),
                                ),
                                stmts: [
                                    ValRepr(
                                        Id {
                                            value: 359,
                                        },
                                    ),
                                ],
                            },
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 360,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 363,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 361,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
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
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 364,
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
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
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