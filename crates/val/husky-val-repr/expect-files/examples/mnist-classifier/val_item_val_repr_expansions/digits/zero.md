[
    (
        FugitivePath(`mnist_classifier::digits::zero::open_one_match`, `Val`),
        None,
    ),
    (
        FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
        Some(
            ValReprExpansion {
                hir_lazy_variable_val_repr_map: ArenaMap {
                    data: [
                        Some(
                            ValRepr(
                                Id {
                                    value: 377,
                                },
                            ),
                        ),
                        Some(
                            ValRepr(
                                Id {
                                    value: 400,
                                },
                            ),
                        ),
                        Some(
                            ValRepr(
                                Id {
                                    value: 410,
                                },
                            ),
                        ),
                        Some(
                            ValRepr(
                                Id {
                                    value: 440,
                                },
                            ),
                        ),
                        Some(
                            ValRepr(
                                Id {
                                    value: 448,
                                },
                            ),
                        ),
                        Some(
                            ValRepr(
                                Id {
                                    value: 454,
                                },
                            ),
                        ),
                        Some(
                            ValRepr(
                                Id {
                                    value: 456,
                                },
                            ),
                        ),
                        Some(
                            ValRepr(
                                Id {
                                    value: 462,
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
                        caching_class: ValItem,
                    },
                    ValRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssociatedItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::connected_component::ConnectedComponent(0)::raw_contours`, `MemoizedField`),
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
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MethodFn {
                                    path: AssociatedItemPath::TypeItem(
                                        TypeItemPath(`(core::vec::Vec(0)::ilen`, `MethodFn`),
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
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 372,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Literal(
                            TermLiteral::I32(
                                1,
                            ),
                        ),
                        arguments: [],
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Binary(
                            Comparison(
                                Eq,
                            ),
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 373,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 374,
                                    },
                                ),
                            ),
                        ],
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::ValItem {
                                    path: FugitivePath(`mnist_classifier::digits::zero::open_one_match`, `Val`),
                                    instantiation: LinkageInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        caching_class: ValItem,
                    },
                    ValRepr {
                        val_domain_repr: ConditionSatisfied(
                            ValRepr(
                                Id {
                                    value: 375,
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
                                        value: 370,
                                    },
                                ),
                            ),
                        ],
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: ConditionSatisfied(
                            ValRepr(
                                Id {
                                    value: 375,
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
                                        value: 370,
                                    },
                                ),
                            ),
                        ],
                        caching_class: Variable,
                    },
                    ValRepr {
                        val_domain_repr: ConditionSatisfied(
                            ValRepr(
                                Id {
                                    value: 375,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            TermLiteral::F32(
                                TermF32Literal {
                                    value: OrderedFloat(
                                        1.5,
                                    ),
                                    text: "1.5f32",
                                },
                            ),
                        ),
                        arguments: [],
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: ConditionSatisfied(
                            ValRepr(
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
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 377,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 379,
                                    },
                                ),
                            ),
                        ],
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::ValItem {
                                    path: FugitivePath(`mnist_classifier::digits::zero::open_one_match`, `Val`),
                                    instantiation: LinkageInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        caching_class: ValItem,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 381,
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
                                        value: 370,
                                    },
                                ),
                            ),
                        ],
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 381,
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
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 381,
                                },
                            ),
                        ),
                        opn: ValOpn::Index,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 383,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 384,
                                    },
                                ),
                            ),
                        ],
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::ValItem {
                                    path: FugitivePath(`mnist_classifier::major::connected_components`, `Val`),
                                    instantiation: LinkageInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        caching_class: ValItem,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 387,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MethodFn {
                                    path: AssociatedItemPath::TypeItem(
                                        TypeItemPath(`(core::vec::Vec(0)::ilen`, `MethodFn`),
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
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 389,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 387,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            TermLiteral::I32(
                                1,
                            ),
                        ),
                        arguments: [],
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
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
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 390,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 391,
                                    },
                                ),
                            ),
                        ],
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::ValItem {
                                    path: FugitivePath(`mnist_classifier::digits::zero::open_one_match`, `Val`),
                                    instantiation: LinkageInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        caching_class: ValItem,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 393,
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
                                        value: 370,
                                    },
                                ),
                            ),
                        ],
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 393,
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
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 393,
                                },
                            ),
                        ),
                        opn: ValOpn::Index,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 394,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 395,
                                    },
                                ),
                            ),
                        ],
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 393,
                                },
                            ),
                        ),
                        opn: ValOpn::Unwrap,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 396,
                                    },
                                ),
                            ),
                        ],
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 393,
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
                                        value: 397,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 393,
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
                                        value: 398,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 393,
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
                                        value: 398,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        caching_class: Variable,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 393,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            TermLiteral::F32(
                                TermF32Literal {
                                    value: OrderedFloat(
                                        5.5,
                                    ),
                                    text: "5.5f32",
                                },
                            ),
                        ),
                        arguments: [],
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
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
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 400,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 402,
                                    },
                                ),
                            ),
                        ],
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 404,
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
                        caching_class: Stmt,
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
                        caching_class: ValItem,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 407,
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
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 407,
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
                                        value: 408,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 407,
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
                                        value: 408,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        caching_class: Variable,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 407,
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
                                        value: 410,
                                    },
                                ),
                            ),
                        ],
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 407,
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
                                        value: 408,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        caching_class: Variable,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 407,
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
                                        value: 410,
                                    },
                                ),
                            ),
                        ],
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 407,
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
                                        value: 408,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        caching_class: Variable,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 407,
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
                                        value: 410,
                                    },
                                ),
                            ),
                        ],
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 407,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            TermLiteral::I32(
                                5,
                            ),
                        ),
                        arguments: [],
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 407,
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
                                            value: 411,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 412,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 413,
                                        },
                                    ),
                                ],
                            ),
                            Keyed(
                                Some(
                                    ValRepr(
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
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 407,
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
                                        value: 415,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 407,
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
                                        value: 408,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        caching_class: Variable,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 416,
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
                                        value: 410,
                                    },
                                ),
                            ),
                        ],
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 416,
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
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
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
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 418,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 419,
                                    },
                                ),
                            ),
                        ],
                        caching_class: Stmt,
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
                        caching_class: ValItem,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 421,
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
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 421,
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
                                        value: 423,
                                    },
                                ),
                            ),
                        ],
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 421,
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
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 421,
                                },
                            ),
                        ),
                        opn: ValOpn::Index,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 424,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 425,
                                    },
                                ),
                            ),
                        ],
                        caching_class: Stmt,
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
                        caching_class: ValItem,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 428,
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
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 428,
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
                                        value: 430,
                                    },
                                ),
                            ),
                        ],
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 428,
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
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 428,
                                },
                            ),
                        ),
                        opn: ValOpn::Index,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 431,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 432,
                                    },
                                ),
                            ),
                        ],
                        caching_class: Stmt,
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
                        caching_class: ValItem,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 435,
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
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 435,
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
                                        value: 436,
                                    },
                                ),
                            ),
                        ],
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 435,
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
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 435,
                                },
                            ),
                        ),
                        opn: ValOpn::Index,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 437,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 438,
                                    },
                                ),
                            ),
                        ],
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 435,
                                },
                            ),
                        ),
                        opn: ValOpn::Index,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 437,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 438,
                                    },
                                ),
                            ),
                        ],
                        caching_class: Variable,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 435,
                                },
                            ),
                        ),
                        opn: ValOpn::Unwrap,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 440,
                                    },
                                ),
                            ),
                        ],
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 435,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssociatedItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::raw_contour::RawContour(0)::bounding_box`, `MemoizedField`),
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
                                        value: 441,
                                    },
                                ),
                            ),
                        ],
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 435,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MethodFn {
                                    path: AssociatedItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::geom2d::BoundingBox(0)::ymax`, `MethodFn`),
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
                                        value: 442,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 435,
                                },
                            ),
                        ),
                        opn: ValOpn::Index,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 437,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 438,
                                    },
                                ),
                            ),
                        ],
                        caching_class: Variable,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 435,
                                },
                            ),
                        ),
                        opn: ValOpn::Unwrap,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 440,
                                    },
                                ),
                            ),
                        ],
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 435,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssociatedItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::raw_contour::RawContour(0)::bounding_box`, `MemoizedField`),
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
                                        value: 444,
                                    },
                                ),
                            ),
                        ],
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 435,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MethodFn {
                                    path: AssociatedItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::geom2d::BoundingBox(0)::ymin`, `MethodFn`),
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
                                        value: 445,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
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
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 443,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 446,
                                    },
                                ),
                            ),
                        ],
                        caching_class: Stmt,
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
                        caching_class: ValItem,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 435,
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
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 435,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MethodFn {
                                    path: AssociatedItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::geom2d::BoundingBox(0)::ymax`, `MethodFn`),
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
                                        value: 449,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        caching_class: Stmt,
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
                        caching_class: ValItem,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 435,
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
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 435,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MethodFn {
                                    path: AssociatedItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::geom2d::BoundingBox(0)::ymin`, `MethodFn`),
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
                                        value: 451,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
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
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 450,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 452,
                                    },
                                ),
                            ),
                        ],
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
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
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 443,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 446,
                                    },
                                ),
                            ),
                        ],
                        caching_class: Variable,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
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
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 450,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 452,
                                    },
                                ),
                            ),
                        ],
                        caching_class: Variable,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
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
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 448,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 454,
                                    },
                                ),
                            ),
                        ],
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
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
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 448,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 454,
                                    },
                                ),
                            ),
                        ],
                        caching_class: Variable,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 435,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            TermLiteral::F32(
                                TermF32Literal {
                                    value: OrderedFloat(
                                        0.4,
                                    ),
                                    text: "0.4f32",
                                },
                            ),
                        ),
                        arguments: [],
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
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
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 456,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 458,
                                    },
                                ),
                            ),
                        ],
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 407,
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
                                        value: 408,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        caching_class: Variable,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 460,
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
                                        value: 410,
                                    },
                                ),
                            ),
                        ],
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 460,
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
                        caching_class: Stmt,
                    },
                ],
                hir_lazy_stmt_val_repr_map: [
                    ValRepr {
                        val_domain_repr: ConditionSatisfied(
                            ValRepr(
                                Id {
                                    value: 375,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 380,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 378,
                                    },
                                ),
                            ),
                        ],
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 381,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 386,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 382,
                                    },
                                ),
                            ),
                        ],
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 387,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 392,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 388,
                                    },
                                ),
                            ),
                        ],
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 393,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 403,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 401,
                                    },
                                ),
                            ),
                        ],
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 404,
                                },
                            ),
                        ),
                        opn: ValOpn::Return,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 405,
                                    },
                                ),
                            ),
                        ],
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Branches,
                        arguments: [
                            Branch {
                                condition: Some(
                                    ValRepr(
                                        Id {
                                            value: 375,
                                        },
                                    ),
                                ),
                                stmts: [
                                    ValRepr(
                                        Id {
                                            value: 381,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 387,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 393,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 404,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 406,
                                        },
                                    ),
                                ],
                            },
                        ],
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 416,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 420,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 417,
                                    },
                                ),
                            ),
                        ],
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 421,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 427,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 422,
                                    },
                                ),
                            ),
                        ],
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 428,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 434,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 429,
                                    },
                                ),
                            ),
                        ],
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 435,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 459,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 457,
                                    },
                                ),
                            ),
                        ],
                        caching_class: Stmt,
                    },
                ],
                root_hir_lazy_stmt_val_reprs: [
                    ValRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Branches,
                        arguments: [
                            Branch {
                                condition: Some(
                                    ValRepr(
                                        Id {
                                            value: 375,
                                        },
                                    ),
                                ),
                                stmts: [
                                    ValRepr(
                                        Id {
                                            value: 381,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 387,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 393,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 404,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 406,
                                        },
                                    ),
                                ],
                            },
                        ],
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 407,
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
                                        value: 415,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 416,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 420,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 417,
                                    },
                                ),
                            ),
                        ],
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 421,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 427,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 422,
                                    },
                                ),
                            ),
                        ],
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 428,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 434,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 429,
                                    },
                                ),
                            ),
                        ],
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 435,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 459,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 457,
                                    },
                                ),
                            ),
                        ],
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 460,
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
                        caching_class: Stmt,
                    },
                ],
            },
        ),
    ),
]