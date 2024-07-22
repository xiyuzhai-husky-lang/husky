```rust
[
    (
        MajorFormPath(`mnist_classifier::digits::six::six_match`, `Val`),
        None,
    ),
    (
        MajorFormPath(`mnist_classifier::digits::six::six_match_refined1`, `Val`),
        None,
    ),
    (
        MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
        Some(
            KiReprExpansion {
                hir_lazy_variable_ki_repr_map: ArenaMap {
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
                hir_lazy_expr_ki_repr_map: [
                    KiRepr {
                        ki_domain_repr: Omni,
                        opn: KiOpn::Linket(
                            Linket {
                                data: LinketData::MajorVal {
                                    path: MajorFormPath(`mnist_classifier::digits::six::six_match`, `Val`),
                                    instantiation: LinInstantiation {
                                        context: LinTypeContext {
                                            comptime_var_overrides: [],
                                        },
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: KiReprSource::Val(
                            MajorFormPath(`mnist_classifier::digits::six::six_match`, `Val`),
                        ),
                        caching_class: Val,
                    },
                    KiRepr {
                        ki_domain_repr: Omni,
                        opn: KiOpn::Linket(
                            Linket {
                                data: LinketData::StructField {
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
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
                            },
                            source: Expr {
                                expr: 1,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: Omni,
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
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
                            },
                            source: Expr {
                                expr: 2,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: Omni,
                        opn: KiOpn::Index,
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
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
                            },
                            source: Expr {
                                expr: 3,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: Omni,
                        opn: KiOpn::Index,
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
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
                            },
                            source: LetVariable {
                                stmt: 19,
                                variable_idx: 0,
                            },
                        },
                        caching_class: Variable,
                    },
                    KiRepr {
                        ki_domain_repr: Omni,
                        opn: KiOpn::Linket(
                            Linket {
                                data: LinketData::MajorVal {
                                    path: MajorFormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                                    instantiation: LinInstantiation {
                                        context: LinTypeContext {
                                            comptime_var_overrides: [],
                                        },
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: KiReprSource::Val(
                            MajorFormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                        ),
                        caching_class: Val,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 251,
                                },
                            ),
                        ),
                        opn: KiOpn::Linket(
                            Linket {
                                data: LinketData::MemoizedField {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(
                                            `mnist_classifier::connected_component::ConnectedComponent(0)::eff_holes`,
                                            TypeItemKind::MemoizedField,
                                        ),
                                    ),
                                    instantiation: LinInstantiation {
                                        context: LinTypeContext {
                                            comptime_var_overrides: [],
                                        },
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
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
                            },
                            source: Expr {
                                expr: 6,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: Omni,
                        opn: KiOpn::Linket(
                            Linket {
                                data: LinketData::MajorVal {
                                    path: MajorFormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                                    instantiation: LinInstantiation {
                                        context: LinTypeContext {
                                            comptime_var_overrides: [],
                                        },
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: KiReprSource::Val(
                            MajorFormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                        ),
                        caching_class: Val,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 251,
                                },
                            ),
                        ),
                        opn: KiOpn::Linket(
                            Linket {
                                data: LinketData::MemoizedField {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(
                                            `mnist_classifier::connected_component::ConnectedComponent(0)::lower_mass`,
                                            TypeItemKind::MemoizedField,
                                        ),
                                    ),
                                    instantiation: LinInstantiation {
                                        context: LinTypeContext {
                                            comptime_var_overrides: [],
                                        },
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
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
                            },
                            source: Expr {
                                expr: 8,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: Omni,
                        opn: KiOpn::Linket(
                            Linket {
                                data: LinketData::MajorVal {
                                    path: MajorFormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                                    instantiation: LinInstantiation {
                                        context: LinTypeContext {
                                            comptime_var_overrides: [],
                                        },
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: KiReprSource::Val(
                            MajorFormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                        ),
                        caching_class: Val,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 251,
                                },
                            ),
                        ),
                        opn: KiOpn::Linket(
                            Linket {
                                data: LinketData::MemoizedField {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(
                                            `mnist_classifier::connected_component::ConnectedComponent(0)::upper_mass`,
                                            TypeItemKind::MemoizedField,
                                        ),
                                    ),
                                    instantiation: LinInstantiation {
                                        context: LinTypeContext {
                                            comptime_var_overrides: [],
                                        },
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
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
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
                                    value: 251,
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
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
                            },
                            source: Expr {
                                expr: 11,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 251,
                                },
                            ),
                        ),
                        opn: KiOpn::Linket(
                            Linket {
                                data: LinketData::MemoizedField {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(
                                            `mnist_classifier::connected_component::ConnectedComponent(0)::eff_holes`,
                                            TypeItemKind::MemoizedField,
                                        ),
                                    ),
                                    instantiation: LinInstantiation {
                                        context: LinTypeContext {
                                            comptime_var_overrides: [],
                                        },
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
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
                            },
                            source: LetVariable {
                                stmt: 21,
                                variable_idx: 1,
                            },
                        },
                        caching_class: Variable,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 251,
                                },
                            ),
                        ),
                        opn: KiOpn::Linket(
                            Linket {
                                data: LinketData::StructField {
                                    self_ty: LinTypePathLeading {
                                        ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                        template_arguments: [
                                            LinTemplateArgument::Type(
                                                LinType::PathLeading(
                                                    LinTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
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
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
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
                                    value: 251,
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
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
                            },
                            source: Expr {
                                expr: 14,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 251,
                                },
                            ),
                        ),
                        opn: KiOpn::Index,
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
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
                            },
                            source: Expr {
                                expr: 15,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: Omni,
                        opn: KiOpn::Linket(
                            Linket {
                                data: LinketData::MajorVal {
                                    path: MajorFormPath(`mnist_classifier::digits::six::six_match`, `Val`),
                                    instantiation: LinInstantiation {
                                        context: LinTypeContext {
                                            comptime_var_overrides: [],
                                        },
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: KiReprSource::Val(
                            MajorFormPath(`mnist_classifier::digits::six::six_match`, `Val`),
                        ),
                        caching_class: Val,
                    },
                    KiRepr {
                        ki_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 261,
                                },
                            ),
                        ),
                        opn: KiOpn::Linket(
                            Linket {
                                data: LinketData::MemoizedField {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(
                                            `mnist_classifier::fermi::FermiMatchResult(0)::norm`,
                                            TypeItemKind::MemoizedField,
                                        ),
                                    ),
                                    instantiation: LinInstantiation {
                                        context: LinTypeContext {
                                            comptime_var_overrides: [],
                                        },
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
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
                            },
                            source: Expr {
                                expr: 18,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: Omni,
                        opn: KiOpn::Linket(
                            Linket {
                                data: LinketData::MajorVal {
                                    path: MajorFormPath(`mnist_classifier::digits::six::six_match_refined1`, `Val`),
                                    instantiation: LinInstantiation {
                                        context: LinTypeContext {
                                            comptime_var_overrides: [],
                                        },
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: KiReprSource::Val(
                            MajorFormPath(`mnist_classifier::digits::six::six_match_refined1`, `Val`),
                        ),
                        caching_class: Val,
                    },
                    KiRepr {
                        ki_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 261,
                                },
                            ),
                        ),
                        opn: KiOpn::Linket(
                            Linket {
                                data: LinketData::MemoizedField {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(
                                            `mnist_classifier::fermi::FermiMatchResult(0)::rel_norm`,
                                            TypeItemKind::MemoizedField,
                                        ),
                                    ),
                                    instantiation: LinInstantiation {
                                        context: LinTypeContext {
                                            comptime_var_overrides: [],
                                        },
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
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
                            },
                            source: Expr {
                                expr: 20,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 261,
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
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
                            },
                            source: Expr {
                                expr: 21,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 261,
                                },
                            ),
                        ),
                        opn: KiOpn::Linket(
                            Linket {
                                data: LinketData::MajorFunctionRitchie {
                                    path: MajorFormPath(`malamute::narrow_down`, `Ritchie(
                                        Gn,
                                    )`),
                                    instantiation: LinInstantiation {
                                        context: LinTypeContext {
                                            comptime_var_overrides: [
                                                (
                                                    MajorItem(
                                                        Form(
                                                            MajorFormPath(
                                                                ItemPathId(
                                                                    Id {
                                                                        value: 116,
                                                                    },
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                    Type(
                                                        PathLeading(
                                                            LinTypePathLeading(
                                                                Id {
                                                                    value: 2,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            ],
                                        },
                                        symbol_resolutions: [
                                            (
                                                Type(
                                                    Type {
                                                        attrs: HirTemplateVariableAttrs {
                                                            class: Mono,
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
                                    KiRuntimeConstant(
                                        Id {
                                            value: 2,
                                        },
                                    ),
                                ],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
                            },
                            source: Expr {
                                expr: 22,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 261,
                                },
                            ),
                        ),
                        opn: KiOpn::Linket(
                            Linket {
                                data: LinketData::UnveilAssocRitchie {
                                    path: TraitForTypeItemPath(
                                        `<malamute::OneVsAll as core::ops::Unveil(0)>::unveil`,
                                        TraitItemKind::AssocRitchie(
                                            RitchieItemKind::Fn,
                                        ),
                                    ),
                                    instantiation: LinInstantiation {
                                        context: LinTypeContext {
                                            comptime_var_overrides: [],
                                        },
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
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
                            },
                            source: Expr {
                                expr: 23,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: Omni,
                        opn: KiOpn::Linket(
                            Linket {
                                data: LinketData::MajorVal {
                                    path: MajorFormPath(`mnist_classifier::digits::six::six_match_refined1`, `Val`),
                                    instantiation: LinInstantiation {
                                        context: LinTypeContext {
                                            comptime_var_overrides: [],
                                        },
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: KiReprSource::Val(
                            MajorFormPath(`mnist_classifier::digits::six::six_match_refined1`, `Val`),
                        ),
                        caching_class: Val,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 266,
                                },
                            ),
                        ),
                        opn: KiOpn::Linket(
                            Linket {
                                data: LinketData::StructField {
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
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
                            },
                            source: Expr {
                                expr: 25,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 266,
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
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
                            },
                            source: Expr {
                                expr: 26,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 266,
                                },
                            ),
                        ),
                        opn: KiOpn::Index,
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
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
                            },
                            source: Expr {
                                expr: 27,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 266,
                                },
                            ),
                        ),
                        opn: KiOpn::Index,
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
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
                            },
                            source: LetVariable {
                                stmt: 6,
                                variable_idx: 3,
                            },
                        },
                        caching_class: Variable,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 266,
                                },
                            ),
                        ),
                        opn: KiOpn::Unwrap,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 270,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
                            },
                            source: Expr {
                                expr: 29,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 266,
                                },
                            ),
                        ),
                        opn: KiOpn::Linket(
                            Linket {
                                data: LinketData::MethodRitchie {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(
                                            `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::displacement`,
                                            TypeItemKind::MethodRitchie(
                                                RitchieItemKind::Fn,
                                            ),
                                        ),
                                    ),
                                    instantiation: LinInstantiation {
                                        context: LinTypeContext {
                                            comptime_var_overrides: [],
                                        },
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
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
                            },
                            source: Expr {
                                expr: 30,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 266,
                                },
                            ),
                        ),
                        opn: KiOpn::Linket(
                            Linket {
                                data: LinketData::MethodRitchie {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(
                                            `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::displacement`,
                                            TypeItemKind::MethodRitchie(
                                                RitchieItemKind::Fn,
                                            ),
                                        ),
                                    ),
                                    instantiation: LinInstantiation {
                                        context: LinTypeContext {
                                            comptime_var_overrides: [],
                                        },
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
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
                            },
                            source: LetVariable {
                                stmt: 7,
                                variable_idx: 4,
                            },
                        },
                        caching_class: Variable,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 266,
                                },
                            ),
                        ),
                        opn: KiOpn::Linket(
                            Linket {
                                data: LinketData::StructField {
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
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
                            },
                            source: Expr {
                                expr: 32,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: Omni,
                        opn: KiOpn::Index,
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
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
                            },
                            source: LetVariable {
                                stmt: 19,
                                variable_idx: 0,
                            },
                        },
                        caching_class: Variable,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 266,
                                },
                            ),
                        ),
                        opn: KiOpn::Unwrap,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 248,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
                            },
                            source: Expr {
                                expr: 34,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 266,
                                },
                            ),
                        ),
                        opn: KiOpn::Linket(
                            Linket {
                                data: LinketData::MethodRitchie {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(
                                            `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::displacement`,
                                            TypeItemKind::MethodRitchie(
                                                RitchieItemKind::Fn,
                                            ),
                                        ),
                                    ),
                                    instantiation: LinInstantiation {
                                        context: LinTypeContext {
                                            comptime_var_overrides: [],
                                        },
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
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
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
                                    value: 266,
                                },
                            ),
                        ),
                        opn: KiOpn::Linket(
                            Linket {
                                data: LinketData::StructField {
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
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
                            },
                            source: Expr {
                                expr: 36,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: Omni,
                        opn: KiOpn::Linket(
                            Linket {
                                data: LinketData::MajorVal {
                                    path: MajorFormPath(`mnist_classifier::digits::six::six_match_refined1`, `Val`),
                                    instantiation: LinInstantiation {
                                        context: LinTypeContext {
                                            comptime_var_overrides: [],
                                        },
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: KiReprSource::Val(
                            MajorFormPath(`mnist_classifier::digits::six::six_match_refined1`, `Val`),
                        ),
                        caching_class: Val,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 266,
                                },
                            ),
                        ),
                        opn: KiOpn::Linket(
                            Linket {
                                data: LinketData::StructField {
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
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
                            },
                            source: Expr {
                                expr: 38,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: Omni,
                        opn: KiOpn::Linket(
                            Linket {
                                data: LinketData::MajorVal {
                                    path: MajorFormPath(`mnist_classifier::digits::six::six_match_refined1`, `Val`),
                                    instantiation: LinInstantiation {
                                        context: LinTypeContext {
                                            comptime_var_overrides: [],
                                        },
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: KiReprSource::Val(
                            MajorFormPath(`mnist_classifier::digits::six::six_match_refined1`, `Val`),
                        ),
                        caching_class: Val,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 266,
                                },
                            ),
                        ),
                        opn: KiOpn::Linket(
                            Linket {
                                data: LinketData::MemoizedField {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(
                                            `mnist_classifier::fermi::FermiMatchResult(0)::norm`,
                                            TypeItemKind::MemoizedField,
                                        ),
                                    ),
                                    instantiation: LinInstantiation {
                                        context: LinTypeContext {
                                            comptime_var_overrides: [],
                                        },
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
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
                            },
                            source: Expr {
                                expr: 40,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 266,
                                },
                            ),
                        ),
                        opn: KiOpn::Literal(
                            Literal::F32(
                                F32Literal {
                                    value: OrderedFloat(
                                        1.8,
                                    ),
                                    text: "1.8f32",
                                },
                            ),
                        ),
                        arguments: [],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
                            },
                            source: Expr {
                                expr: 41,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 266,
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
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
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
                                    value: 266,
                                },
                            ),
                        ),
                        opn: KiOpn::Index,
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
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
                            },
                            source: LetVariable {
                                stmt: 6,
                                variable_idx: 3,
                            },
                        },
                        caching_class: Variable,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 266,
                                },
                            ),
                        ),
                        opn: KiOpn::Linket(
                            Linket {
                                data: LinketData::StructField {
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
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
                            },
                            source: LetVariable {
                                stmt: 8,
                                variable_idx: 5,
                            },
                        },
                        caching_class: Variable,
                    },
                    KiRepr {
                        ki_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 287,
                                },
                            ),
                        ),
                        opn: KiOpn::Literal(
                            Literal::F32(
                                F32Literal {
                                    value: OrderedFloat(
                                        2.5,
                                    ),
                                    text: "2.5f32",
                                },
                            ),
                        ),
                        arguments: [],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
                            },
                            source: Expr {
                                expr: 45,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 287,
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
                                        value: 289,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
                            },
                            source: Expr {
                                expr: 46,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 287,
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
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
                            },
                            source: Expr {
                                expr: 47,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: Omni,
                        opn: KiOpn::Linket(
                            Linket {
                                data: LinketData::MajorVal {
                                    path: MajorFormPath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Val`),
                                    instantiation: LinInstantiation {
                                        context: LinTypeContext {
                                            comptime_var_overrides: [],
                                        },
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: KiReprSource::Val(
                            MajorFormPath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Val`),
                        ),
                        caching_class: Val,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 292,
                                },
                            ),
                        ),
                        opn: KiOpn::Literal(
                            Literal::F32(
                                F32Literal {
                                    value: OrderedFloat(
                                        30.0,
                                    ),
                                    text: "30.0f32",
                                },
                            ),
                        ),
                        arguments: [],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
                            },
                            source: Expr {
                                expr: 49,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 292,
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
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
                            },
                            source: Expr {
                                expr: 50,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 266,
                                },
                            ),
                        ),
                        opn: KiOpn::Linket(
                            Linket {
                                data: LinketData::StructField {
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
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
                            },
                            source: LetVariable {
                                stmt: 9,
                                variable_idx: 6,
                            },
                        },
                        caching_class: Variable,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 296,
                                },
                            ),
                        ),
                        opn: KiOpn::Literal(
                            Literal::I32(
                                15,
                            ),
                        ),
                        arguments: [],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
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
                                    value: 296,
                                },
                            ),
                        ),
                        opn: KiOpn::Linket(
                            Linket {
                                data: LinketData::MajorFunctionRitchie {
                                    path: MajorFormPath(`malamute::narrow_down`, `Ritchie(
                                        Gn,
                                    )`),
                                    instantiation: LinInstantiation {
                                        context: LinTypeContext {
                                            comptime_var_overrides: [
                                                (
                                                    MajorItem(
                                                        Form(
                                                            MajorFormPath(
                                                                ItemPathId(
                                                                    Id {
                                                                        value: 116,
                                                                    },
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                    Type(
                                                        PathLeading(
                                                            LinTypePathLeading(
                                                                Id {
                                                                    value: 2,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            ],
                                        },
                                        symbol_resolutions: [
                                            (
                                                Type(
                                                    Type {
                                                        attrs: HirTemplateVariableAttrs {
                                                            class: Mono,
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
                                    KiRuntimeConstant(
                                        Id {
                                            value: 2,
                                        },
                                    ),
                                ],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
                            },
                            source: Expr {
                                expr: 54,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 296,
                                },
                            ),
                        ),
                        opn: KiOpn::Linket(
                            Linket {
                                data: LinketData::UnveilAssocRitchie {
                                    path: TraitForTypeItemPath(
                                        `<malamute::OneVsAll as core::ops::Unveil(0)>::unveil`,
                                        TraitItemKind::AssocRitchie(
                                            RitchieItemKind::Fn,
                                        ),
                                    ),
                                    instantiation: LinInstantiation {
                                        context: LinTypeContext {
                                            comptime_var_overrides: [],
                                        },
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
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
                            },
                            source: Expr {
                                expr: 55,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 299,
                                },
                            ),
                        ),
                        opn: KiOpn::TypeVariant(
                            TypeVariantPath(`malamute::OneVsAll::Yes`),
                        ),
                        arguments: [],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
                            },
                            source: Expr {
                                expr: 56,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: Omni,
                        opn: KiOpn::Linket(
                            Linket {
                                data: LinketData::MajorVal {
                                    path: MajorFormPath(`mnist_classifier::major::major_line_segment_sketch`, `Val`),
                                    instantiation: LinInstantiation {
                                        context: LinTypeContext {
                                            comptime_var_overrides: [],
                                        },
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: KiReprSource::Val(
                            MajorFormPath(`mnist_classifier::major::major_line_segment_sketch`, `Val`),
                        ),
                        caching_class: Val,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 302,
                                },
                            ),
                        ),
                        opn: KiOpn::Linket(
                            Linket {
                                data: LinketData::MemoizedField {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(
                                            `mnist_classifier::line_segment_sketch::LineSegmentSketch(0)::bounding_box`,
                                            TypeItemKind::MemoizedField,
                                        ),
                                    ),
                                    instantiation: LinInstantiation {
                                        context: LinTypeContext {
                                            comptime_var_overrides: [],
                                        },
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
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
                            },
                            source: Expr {
                                expr: 58,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: Omni,
                        opn: KiOpn::Index,
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
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
                            },
                            source: LetVariable {
                                stmt: 19,
                                variable_idx: 0,
                            },
                        },
                        caching_class: Variable,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 302,
                                },
                            ),
                        ),
                        opn: KiOpn::Unwrap,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 248,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
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
                                    value: 302,
                                },
                            ),
                        ),
                        opn: KiOpn::Linket(
                            Linket {
                                data: LinketData::MethodRitchie {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(
                                            `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::end`,
                                            TypeItemKind::MethodRitchie(
                                                RitchieItemKind::Fn,
                                            ),
                                        ),
                                    ),
                                    instantiation: LinInstantiation {
                                        context: LinTypeContext {
                                            comptime_var_overrides: [],
                                        },
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
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
                            },
                            source: Expr {
                                expr: 61,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 302,
                                },
                            ),
                        ),
                        opn: KiOpn::Linket(
                            Linket {
                                data: LinketData::MethodRitchie {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(
                                            `mnist_classifier::geom2d::BoundingBox(0)::relative_point`,
                                            TypeItemKind::MethodRitchie(
                                                RitchieItemKind::Fn,
                                            ),
                                        ),
                                    ),
                                    instantiation: LinInstantiation {
                                        context: LinTypeContext {
                                            comptime_var_overrides: [],
                                        },
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
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
                            },
                            source: Expr {
                                expr: 62,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 266,
                                },
                            ),
                        ),
                        opn: KiOpn::Linket(
                            Linket {
                                data: LinketData::StructField {
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
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
                            },
                            source: LetVariable {
                                stmt: 9,
                                variable_idx: 6,
                            },
                        },
                        caching_class: Variable,
                    },
                    KiRepr {
                        ki_domain_repr: Omni,
                        opn: KiOpn::Linket(
                            Linket {
                                data: LinketData::MajorVal {
                                    path: MajorFormPath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Val`),
                                    instantiation: LinInstantiation {
                                        context: LinTypeContext {
                                            comptime_var_overrides: [],
                                        },
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: KiReprSource::Val(
                            MajorFormPath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Val`),
                        ),
                        caching_class: Val,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 251,
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
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
                            },
                            source: LetVariable {
                                stmt: 22,
                                variable_idx: 2,
                            },
                        },
                        caching_class: Variable,
                    },
                    KiRepr {
                        ki_domain_repr: Omni,
                        opn: KiOpn::Linket(
                            Linket {
                                data: LinketData::MajorVal {
                                    path: MajorFormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                                    instantiation: LinInstantiation {
                                        context: LinTypeContext {
                                            comptime_var_overrides: [],
                                        },
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: KiReprSource::Val(
                            MajorFormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                        ),
                        caching_class: Val,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 302,
                                },
                            ),
                        ),
                        opn: KiOpn::Literal(
                            Literal::I32(
                                6,
                            ),
                        ),
                        arguments: [],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
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
                                    value: 302,
                                },
                            ),
                        ),
                        opn: KiOpn::Linket(
                            Linket {
                                data: LinketData::MethodRitchie {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(
                                            `mnist_classifier::connected_component::ConnectedComponent(0)::top_k_row_span_sum`,
                                            TypeItemKind::MethodRitchie(
                                                RitchieItemKind::Fn,
                                            ),
                                        ),
                                    ),
                                    instantiation: LinInstantiation {
                                        context: LinTypeContext {
                                            comptime_var_overrides: [],
                                        },
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
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
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
                                    value: 302,
                                },
                            ),
                        ),
                        opn: KiOpn::Literal(
                            Literal::I32(
                                15,
                            ),
                        ),
                        arguments: [],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
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
                                    value: 302,
                                },
                            ),
                        ),
                        opn: KiOpn::Linket(
                            Linket {
                                data: LinketData::MajorFunctionRitchie {
                                    path: MajorFormPath(`malamute::narrow_down`, `Ritchie(
                                        Gn,
                                    )`),
                                    instantiation: LinInstantiation {
                                        context: LinTypeContext {
                                            comptime_var_overrides: [
                                                (
                                                    MajorItem(
                                                        Form(
                                                            MajorFormPath(
                                                                ItemPathId(
                                                                    Id {
                                                                        value: 116,
                                                                    },
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                    Type(
                                                        PathLeading(
                                                            LinTypePathLeading(
                                                                Id {
                                                                    value: 2,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            ],
                                        },
                                        symbol_resolutions: [
                                            (
                                                Type(
                                                    Type {
                                                        attrs: HirTemplateVariableAttrs {
                                                            class: Mono,
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
                                    KiRuntimeConstant(
                                        Id {
                                            value: 2,
                                        },
                                    ),
                                ],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
                            },
                            source: Expr {
                                expr: 71,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 302,
                                },
                            ),
                        ),
                        opn: KiOpn::Linket(
                            Linket {
                                data: LinketData::UnveilAssocRitchie {
                                    path: TraitForTypeItemPath(
                                        `<malamute::OneVsAll as core::ops::Unveil(0)>::unveil`,
                                        TraitItemKind::AssocRitchie(
                                            RitchieItemKind::Fn,
                                        ),
                                    ),
                                    instantiation: LinInstantiation {
                                        context: LinTypeContext {
                                            comptime_var_overrides: [],
                                        },
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
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
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
                                    value: 302,
                                },
                            ),
                        ),
                        opn: KiOpn::Linket(
                            Linket {
                                data: LinketData::MethodRitchie {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(
                                            `mnist_classifier::geom2d::BoundingBox(0)::relative_point`,
                                            TypeItemKind::MethodRitchie(
                                                RitchieItemKind::Fn,
                                            ),
                                        ),
                                    ),
                                    instantiation: LinInstantiation {
                                        context: LinTypeContext {
                                            comptime_var_overrides: [],
                                        },
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
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
                            },
                            source: LetVariable {
                                stmt: 13,
                                variable_idx: 8,
                            },
                        },
                        caching_class: Variable,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 313,
                                },
                            ),
                        ),
                        opn: KiOpn::Linket(
                            Linket {
                                data: LinketData::StructField {
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
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
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
                                    value: 313,
                                },
                            ),
                        ),
                        opn: KiOpn::Literal(
                            Literal::F32(
                                F32Literal {
                                    value: OrderedFloat(
                                        0.7,
                                    ),
                                    text: "0.7f32",
                                },
                            ),
                        ),
                        arguments: [],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
                            },
                            source: Expr {
                                expr: 75,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 313,
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
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
                            },
                            source: Expr {
                                expr: 76,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 316,
                                },
                            ),
                        ),
                        opn: KiOpn::TypeVariant(
                            TypeVariantPath(`malamute::OneVsAll::Yes`),
                        ),
                        arguments: [],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
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
                                    value: 319,
                                },
                            ),
                        ),
                        opn: KiOpn::TypeVariant(
                            TypeVariantPath(`malamute::OneVsAll::Yes`),
                        ),
                        arguments: [],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
                            },
                            source: Expr {
                                expr: 78,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: Omni,
                        opn: KiOpn::Linket(
                            Linket {
                                data: LinketData::MajorVal {
                                    path: MajorFormPath(`mnist_classifier::digits::six::six_match`, `Val`),
                                    instantiation: LinInstantiation {
                                        context: LinTypeContext {
                                            comptime_var_overrides: [],
                                        },
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: KiReprSource::Val(
                            MajorFormPath(`mnist_classifier::digits::six::six_match`, `Val`),
                        ),
                        caching_class: Val,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 322,
                                },
                            ),
                        ),
                        opn: KiOpn::Linket(
                            Linket {
                                data: LinketData::MemoizedField {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(
                                            `mnist_classifier::fermi::FermiMatchResult(0)::norm`,
                                            TypeItemKind::MemoizedField,
                                        ),
                                    ),
                                    instantiation: LinInstantiation {
                                        context: LinTypeContext {
                                            comptime_var_overrides: [],
                                        },
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
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
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
                                    value: 322,
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
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
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
                                    value: 322,
                                },
                            ),
                        ),
                        opn: KiOpn::Linket(
                            Linket {
                                data: LinketData::MajorFunctionRitchie {
                                    path: MajorFormPath(`malamute::narrow_down`, `Ritchie(
                                        Gn,
                                    )`),
                                    instantiation: LinInstantiation {
                                        context: LinTypeContext {
                                            comptime_var_overrides: [
                                                (
                                                    MajorItem(
                                                        Form(
                                                            MajorFormPath(
                                                                ItemPathId(
                                                                    Id {
                                                                        value: 116,
                                                                    },
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                    Type(
                                                        PathLeading(
                                                            LinTypePathLeading(
                                                                Id {
                                                                    value: 2,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            ],
                                        },
                                        symbol_resolutions: [
                                            (
                                                Type(
                                                    Type {
                                                        attrs: HirTemplateVariableAttrs {
                                                            class: Mono,
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
                                    KiRuntimeConstant(
                                        Id {
                                            value: 2,
                                        },
                                    ),
                                ],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
                            },
                            source: Expr {
                                expr: 83,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 322,
                                },
                            ),
                        ),
                        opn: KiOpn::Linket(
                            Linket {
                                data: LinketData::UnveilAssocRitchie {
                                    path: TraitForTypeItemPath(
                                        `<malamute::OneVsAll as core::ops::Unveil(0)>::unveil`,
                                        TraitItemKind::AssocRitchie(
                                            RitchieItemKind::Fn,
                                        ),
                                    ),
                                    instantiation: LinInstantiation {
                                        context: LinTypeContext {
                                            comptime_var_overrides: [],
                                        },
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
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
                            },
                            source: Expr {
                                expr: 84,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: Omni,
                        opn: KiOpn::Linket(
                            Linket {
                                data: LinketData::MajorVal {
                                    path: MajorFormPath(`mnist_classifier::digits::six::six_match`, `Val`),
                                    instantiation: LinInstantiation {
                                        context: LinTypeContext {
                                            comptime_var_overrides: [],
                                        },
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: KiReprSource::Val(
                            MajorFormPath(`mnist_classifier::digits::six::six_match`, `Val`),
                        ),
                        caching_class: Val,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 326,
                                },
                            ),
                        ),
                        opn: KiOpn::Linket(
                            Linket {
                                data: LinketData::MemoizedField {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(
                                            `mnist_classifier::fermi::FermiMatchResult(0)::norm`,
                                            TypeItemKind::MemoizedField,
                                        ),
                                    ),
                                    instantiation: LinInstantiation {
                                        context: LinTypeContext {
                                            comptime_var_overrides: [],
                                        },
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
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
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
                                    value: 326,
                                },
                            ),
                        ),
                        opn: KiOpn::Literal(
                            Literal::F32(
                                F32Literal {
                                    value: OrderedFloat(
                                        1.8,
                                    ),
                                    text: "1.8f32",
                                },
                            ),
                        ),
                        arguments: [],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
                            },
                            source: Expr {
                                expr: 87,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 326,
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
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
                            },
                            source: Expr {
                                expr: 88,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: Omni,
                        opn: KiOpn::Linket(
                            Linket {
                                data: LinketData::MajorVal {
                                    path: MajorFormPath(`mnist_classifier::digits::six::six_match`, `Val`),
                                    instantiation: LinInstantiation {
                                        context: LinTypeContext {
                                            comptime_var_overrides: [],
                                        },
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: KiReprSource::Val(
                            MajorFormPath(`mnist_classifier::digits::six::six_match`, `Val`),
                        ),
                        caching_class: Val,
                    },
                    KiRepr {
                        ki_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 329,
                                },
                            ),
                        ),
                        opn: KiOpn::Linket(
                            Linket {
                                data: LinketData::MemoizedField {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(
                                            `mnist_classifier::fermi::FermiMatchResult(0)::norm`,
                                            TypeItemKind::MemoizedField,
                                        ),
                                    ),
                                    instantiation: LinInstantiation {
                                        context: LinTypeContext {
                                            comptime_var_overrides: [],
                                        },
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
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
                            },
                            source: Expr {
                                expr: 90,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 329,
                                },
                            ),
                        ),
                        opn: KiOpn::Literal(
                            Literal::F32(
                                F32Literal {
                                    value: OrderedFloat(
                                        1.8,
                                    ),
                                    text: "1.8f32",
                                },
                            ),
                        ),
                        arguments: [],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
                            },
                            source: Expr {
                                expr: 91,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 329,
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
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
                            },
                            source: Expr {
                                expr: 92,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 251,
                                },
                            ),
                        ),
                        opn: KiOpn::Linket(
                            Linket {
                                data: LinketData::MemoizedField {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(
                                            `mnist_classifier::connected_component::ConnectedComponent(0)::eff_holes`,
                                            TypeItemKind::MemoizedField,
                                        ),
                                    ),
                                    instantiation: LinInstantiation {
                                        context: LinTypeContext {
                                            comptime_var_overrides: [],
                                        },
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
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
                            },
                            source: LetVariable {
                                stmt: 21,
                                variable_idx: 1,
                            },
                        },
                        caching_class: Variable,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 335,
                                },
                            ),
                        ),
                        opn: KiOpn::Linket(
                            Linket {
                                data: LinketData::StructField {
                                    self_ty: LinTypePathLeading {
                                        ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                        template_arguments: [
                                            LinTemplateArgument::Type(
                                                LinType::PathLeading(
                                                    LinTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
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
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
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
                                    value: 335,
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
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
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
                                    value: 335,
                                },
                            ),
                        ),
                        opn: KiOpn::Index,
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
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
                            },
                            source: Expr {
                                expr: 96,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 335,
                                },
                            ),
                        ),
                        opn: KiOpn::Unwrap,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 339,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
                            },
                            source: Expr {
                                expr: 97,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 335,
                                },
                            ),
                        ),
                        opn: KiOpn::Linket(
                            Linket {
                                data: LinketData::MemoizedField {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(
                                            `mnist_classifier::raw_contour::RawContour(0)::relative_bounding_box`,
                                            TypeItemKind::MemoizedField,
                                        ),
                                    ),
                                    instantiation: LinInstantiation {
                                        context: LinTypeContext {
                                            comptime_var_overrides: [],
                                        },
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
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
                            },
                            source: Expr {
                                expr: 98,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 335,
                                },
                            ),
                        ),
                        opn: KiOpn::Linket(
                            Linket {
                                data: LinketData::MethodRitchie {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(
                                            `mnist_classifier::geom2d::RelativeBoundingBox(0)::ymax`,
                                            TypeItemKind::MethodRitchie(
                                                RitchieItemKind::Fn,
                                            ),
                                        ),
                                    ),
                                    instantiation: LinInstantiation {
                                        context: LinTypeContext {
                                            comptime_var_overrides: [],
                                        },
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
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
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
                                    value: 335,
                                },
                            ),
                        ),
                        opn: KiOpn::Literal(
                            Literal::F32(
                                F32Literal {
                                    value: OrderedFloat(
                                        0.75,
                                    ),
                                    text: "0.75f32",
                                },
                            ),
                        ),
                        arguments: [],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
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
                                    value: 335,
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
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
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
                                    value: 251,
                                },
                            ),
                        ),
                        opn: KiOpn::Linket(
                            Linket {
                                data: LinketData::MemoizedField {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(
                                            `mnist_classifier::connected_component::ConnectedComponent(0)::eff_holes`,
                                            TypeItemKind::MemoizedField,
                                        ),
                                    ),
                                    instantiation: LinInstantiation {
                                        context: LinTypeContext {
                                            comptime_var_overrides: [],
                                        },
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
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
                            },
                            source: LetVariable {
                                stmt: 21,
                                variable_idx: 1,
                            },
                        },
                        caching_class: Variable,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 345,
                                },
                            ),
                        ),
                        opn: KiOpn::Linket(
                            Linket {
                                data: LinketData::StructField {
                                    self_ty: LinTypePathLeading {
                                        ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                        template_arguments: [
                                            LinTemplateArgument::Type(
                                                LinType::PathLeading(
                                                    LinTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
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
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
                            },
                            source: Expr {
                                expr: 103,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 345,
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
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
                            },
                            source: Expr {
                                expr: 104,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 345,
                                },
                            ),
                        ),
                        opn: KiOpn::Index,
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
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
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
                                    value: 251,
                                },
                            ),
                        ),
                        opn: KiOpn::Linket(
                            Linket {
                                data: LinketData::MemoizedField {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(
                                            `mnist_classifier::connected_component::ConnectedComponent(0)::eff_holes`,
                                            TypeItemKind::MemoizedField,
                                        ),
                                    ),
                                    instantiation: LinInstantiation {
                                        context: LinTypeContext {
                                            comptime_var_overrides: [],
                                        },
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
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
                            },
                            source: LetVariable {
                                stmt: 21,
                                variable_idx: 1,
                            },
                        },
                        caching_class: Variable,
                    },
                    KiRepr {
                        ki_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 349,
                                },
                            ),
                        ),
                        opn: KiOpn::Linket(
                            Linket {
                                data: LinketData::StructField {
                                    self_ty: LinTypePathLeading {
                                        ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                        template_arguments: [
                                            LinTemplateArgument::Type(
                                                LinType::PathLeading(
                                                    LinTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
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
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
                            },
                            source: Expr {
                                expr: 107,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 349,
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
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
                            },
                            source: Expr {
                                expr: 108,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 349,
                                },
                            ),
                        ),
                        opn: KiOpn::Index,
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
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
                            },
                            source: Expr {
                                expr: 109,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 349,
                                },
                            ),
                        ),
                        opn: KiOpn::Unwrap,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 353,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
                            },
                            source: Expr {
                                expr: 110,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 349,
                                },
                            ),
                        ),
                        opn: KiOpn::Linket(
                            Linket {
                                data: LinketData::MemoizedField {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(
                                            `mnist_classifier::raw_contour::RawContour(0)::relative_bounding_box`,
                                            TypeItemKind::MemoizedField,
                                        ),
                                    ),
                                    instantiation: LinInstantiation {
                                        context: LinTypeContext {
                                            comptime_var_overrides: [],
                                        },
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
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
                            },
                            source: Expr {
                                expr: 111,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 349,
                                },
                            ),
                        ),
                        opn: KiOpn::Linket(
                            Linket {
                                data: LinketData::MethodRitchie {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(
                                            `mnist_classifier::geom2d::RelativeBoundingBox(0)::ymax`,
                                            TypeItemKind::MethodRitchie(
                                                RitchieItemKind::Fn,
                                            ),
                                        ),
                                    ),
                                    instantiation: LinInstantiation {
                                        context: LinTypeContext {
                                            comptime_var_overrides: [],
                                        },
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
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
                            },
                            source: Expr {
                                expr: 112,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 349,
                                },
                            ),
                        ),
                        opn: KiOpn::Literal(
                            Literal::F32(
                                F32Literal {
                                    value: OrderedFloat(
                                        0.75,
                                    ),
                                    text: "0.75f32",
                                },
                            ),
                        ),
                        arguments: [],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
                            },
                            source: Expr {
                                expr: 113,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 349,
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
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
                            },
                            source: Expr {
                                expr: 114,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 251,
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
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
                            },
                            source: LetVariable {
                                stmt: 22,
                                variable_idx: 2,
                            },
                        },
                        caching_class: Variable,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 360,
                                },
                            ),
                        ),
                        opn: KiOpn::Literal(
                            Literal::F32(
                                F32Literal {
                                    value: OrderedFloat(
                                        15.0,
                                    ),
                                    text: "15.0f32",
                                },
                            ),
                        ),
                        arguments: [],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
                            },
                            source: Expr {
                                expr: 116,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 360,
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
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
                            },
                            source: Expr {
                                expr: 117,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 364,
                                },
                            ),
                        ),
                        opn: KiOpn::TypeVariant(
                            TypeVariantPath(`malamute::OneVsAll::Yes`),
                        ),
                        arguments: [],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
                            },
                            source: Expr {
                                expr: 118,
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
                                    value: 287,
                                },
                            ),
                        ),
                        opn: KiOpn::Require,
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
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
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
                                    value: 292,
                                },
                            ),
                        ),
                        opn: KiOpn::Require,
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
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
                            },
                            source: Stmt {
                                stmt: 1,
                            },
                        },
                        caching_class: Stmt,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 299,
                                },
                            ),
                        ),
                        opn: KiOpn::Return,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 300,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
                            },
                            source: Stmt {
                                stmt: 3,
                            },
                        },
                        caching_class: Stmt,
                    },
                    KiRepr {
                        ki_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 316,
                                },
                            ),
                        ),
                        opn: KiOpn::Return,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 317,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
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
                                    value: 266,
                                },
                            ),
                        ),
                        opn: KiOpn::Require,
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
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
                            },
                            source: Stmt {
                                stmt: 11,
                            },
                        },
                        caching_class: Stmt,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 286,
                                },
                            ),
                        ),
                        opn: KiOpn::Branches,
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
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
                            },
                            source: Stmt {
                                stmt: 12,
                            },
                        },
                        caching_class: Stmt,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 313,
                                },
                            ),
                        ),
                        opn: KiOpn::Branches,
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
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
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
                                    value: 319,
                                },
                            ),
                        ),
                        opn: KiOpn::Return,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 320,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
                            },
                            source: Stmt {
                                stmt: 16,
                            },
                        },
                        caching_class: Stmt,
                    },
                    KiRepr {
                        ki_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 329,
                                },
                            ),
                        ),
                        opn: KiOpn::Require,
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
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
                            },
                            source: Stmt {
                                stmt: 17,
                            },
                        },
                        caching_class: Stmt,
                    },
                    KiRepr {
                        ki_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 349,
                                },
                            ),
                        ),
                        opn: KiOpn::Require,
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
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
                            },
                            source: Stmt {
                                stmt: 18,
                            },
                        },
                        caching_class: Stmt,
                    },
                    KiRepr {
                        ki_domain_repr: Omni,
                        opn: KiOpn::Require,
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
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
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
                                    value: 251,
                                },
                            ),
                        ),
                        opn: KiOpn::Branches,
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
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
                            },
                            source: Stmt {
                                stmt: 23,
                            },
                        },
                        caching_class: Stmt,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 326,
                                },
                            ),
                        ),
                        opn: KiOpn::Branches,
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
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
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
                                    value: 335,
                                },
                            ),
                        ),
                        opn: KiOpn::Require,
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
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
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
                                    value: 345,
                                },
                            ),
                        ),
                        opn: KiOpn::Branches,
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
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
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
                                    value: 360,
                                },
                            ),
                        ),
                        opn: KiOpn::Require,
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
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
                            },
                            source: Stmt {
                                stmt: 28,
                            },
                        },
                        caching_class: Stmt,
                    },
                ],
                root_hir_lazy_stmt_ki_reprs: [
                    KiRepr {
                        ki_domain_repr: Omni,
                        opn: KiOpn::Require,
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
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
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
                                    value: 251,
                                },
                            ),
                        ),
                        opn: KiOpn::Branches,
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
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
                            },
                            source: Stmt {
                                stmt: 23,
                            },
                        },
                        caching_class: Stmt,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 322,
                                },
                            ),
                        ),
                        opn: KiOpn::Linket(
                            Linket {
                                data: LinketData::UnveilAssocRitchie {
                                    path: TraitForTypeItemPath(
                                        `<malamute::OneVsAll as core::ops::Unveil(0)>::unveil`,
                                        TraitItemKind::AssocRitchie(
                                            RitchieItemKind::Fn,
                                        ),
                                    ),
                                    instantiation: LinInstantiation {
                                        context: LinTypeContext {
                                            comptime_var_overrides: [],
                                        },
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
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
                            },
                            source: Expr {
                                expr: 84,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 326,
                                },
                            ),
                        ),
                        opn: KiOpn::Branches,
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
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
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
                                    value: 335,
                                },
                            ),
                        ),
                        opn: KiOpn::Require,
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
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
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
                                    value: 345,
                                },
                            ),
                        ),
                        opn: KiOpn::Branches,
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
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
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
                                    value: 360,
                                },
                            ),
                        ),
                        opn: KiOpn::Require,
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
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
                            },
                            source: Stmt {
                                stmt: 28,
                            },
                        },
                        caching_class: Stmt,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 364,
                                },
                            ),
                        ),
                        opn: KiOpn::TypeVariant(
                            TypeVariantPath(`malamute::OneVsAll::Yes`),
                        ),
                        arguments: [],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValLazilyDefined(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::Val(
                                    MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                caching_class: Val,
                            },
                            source: Expr {
                                expr: 118,
                            },
                        },
                        caching_class: Expr,
                    },
                ],
            },
        ),
    ),
]
```