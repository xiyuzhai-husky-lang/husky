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
                        caching_class: Stmt,
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
                        caching_class: Stmt,
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
                        caching_class: Stmt,
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
                        caching_class: Stmt,
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
                        caching_class: Stmt,
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
                        caching_class: Stmt,
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
                        caching_class: Stmt,
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
                        caching_class: Stmt,
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
                        caching_class: Stmt,
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
                        caching_class: Stmt,
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
                        caching_class: Stmt,
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
                        caching_class: Stmt,
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
                        caching_class: Stmt,
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
                        caching_class: Stmt,
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
                        caching_class: Stmt,
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
                        caching_class: Stmt,
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
                        caching_class: Stmt,
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
                        caching_class: Stmt,
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
                        caching_class: Stmt,
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
                        caching_class: Stmt,
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
                        caching_class: Stmt,
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
                        caching_class: Stmt,
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
                        caching_class: Stmt,
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
                        caching_class: Stmt,
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
                        caching_class: Stmt,
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
                        caching_class: Stmt,
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
                        caching_class: Stmt,
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
                        caching_class: Stmt,
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
                        caching_class: Stmt,
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
                        caching_class: Stmt,
                    },
                ],
            },
        ),
    ),
]