[
    (
        FugitivePath(`mnist_classifier::digits::zero::open_one_match`, `Val`),
        None,
    ),
    (
        FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
        Some(
            ValReprExpansion {
                [salsa id]: 2,
                hir_lazy_variable_val_repr_map: ArenaMap {
                    data: [
                        None,
                        Some(
                            ValRepr(
                                Id {
                                    value: 361,
                                },
                            ),
                        ),
                        None,
                        Some(
                            ValRepr(
                                Id {
                                    value: 381,
                                },
                            ),
                        ),
                        Some(
                            ValRepr(
                                Id {
                                    value: 390,
                                },
                            ),
                        ),
                        None,
                        None,
                        Some(
                            ValRepr(
                                Id {
                                    value: 417,
                                },
                            ),
                        ),
                        Some(
                            ValRepr(
                                Id {
                                    value: 425,
                                },
                            ),
                        ),
                        Some(
                            ValRepr(
                                Id {
                                    value: 431,
                                },
                            ),
                        ),
                        Some(
                            ValRepr(
                                Id {
                                    value: 433,
                                },
                            ),
                        ),
                        Some(
                            ValRepr(
                                Id {
                                    value: 438,
                                },
                            ),
                        ),
                    ],
                },
                hir_lazy_expr_val_repr_map: [
                    ValRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::ValItem(
                            FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                        ),
                        arguments: [],
                        caching_class: ValItem,
                    },
                    ValRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Be,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 13,
                                    },
                                ),
                            ),
                        ],
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::ValItem(
                            FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                        ),
                        arguments: [],
                        caching_class: ValItem,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 355,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            LinkagePath {
                                data: Field,
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
                                    value: 356,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            LinkagePath {
                                data: Method,
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 356,
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
                                    value: 357,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            I32(
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
                                    value: 358,
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
                                        value: 357,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 358,
                                    },
                                ),
                            ),
                        ],
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::ValItem(
                            FugitivePath(`mnist_classifier::digits::zero::open_one_match`, `Val`),
                        ),
                        arguments: [],
                        caching_class: ValItem,
                    },
                    ValRepr {
                        val_domain_repr: ConditionSatisfied(
                            ValRepr(
                                Id {
                                    value: 359,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            LinkagePath {
                                data: Field,
                            },
                        ),
                        arguments: [
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
                                    value: 359,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            LinkagePath {
                                data: Field,
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 352,
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
                            F32(
                                NotNan(
                                    1.5,
                                ),
                            ),
                        ),
                        arguments: [],
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 362,
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
                                        value: 361,
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
                        val_domain_repr: Omni,
                        opn: ValOpn::ValItem(
                            FugitivePath(`mnist_classifier::digits::zero::open_one_match`, `Val`),
                        ),
                        arguments: [],
                        caching_class: ValItem,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 364,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            LinkagePath {
                                data: Field,
                            },
                        ),
                        arguments: [
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
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 365,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            USize(
                                TermUSizeLiteral(
                                    Id {
                                        value: 2,
                                    },
                                ),
                            ),
                        ),
                        arguments: [],
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 366,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            LinkagePath {
                                data: Index,
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 365,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 366,
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
                                    value: 367,
                                },
                            ),
                        ),
                        opn: ValOpn::Be,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 367,
                                    },
                                ),
                            ),
                        ],
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::ValItem(
                            FugitivePath(`mnist_classifier::major::connected_components`, `Val`),
                        ),
                        arguments: [],
                        caching_class: ValItem,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 369,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            LinkagePath {
                                data: Method,
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
                                    value: 371,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            I32(
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
                                    value: 372,
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
                                        value: 371,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 372,
                                    },
                                ),
                            ),
                        ],
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::ValItem(
                            FugitivePath(`mnist_classifier::digits::zero::open_one_match`, `Val`),
                        ),
                        arguments: [],
                        caching_class: ValItem,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 374,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            LinkagePath {
                                data: Field,
                            },
                        ),
                        arguments: [
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
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 375,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            USize(
                                TermUSizeLiteral(
                                    Id {
                                        value: 2,
                                    },
                                ),
                            ),
                        ),
                        arguments: [],
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 376,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            LinkagePath {
                                data: Index,
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 375,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 376,
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
                                    value: 377,
                                },
                            ),
                        ),
                        opn: ValOpn::Suffix(
                            Unwrap,
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 377,
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
                                    value: 378,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            LinkagePath {
                                data: Method,
                            },
                        ),
                        arguments: [
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
                                    value: 379,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            LinkagePath {
                                data: Method,
                            },
                        ),
                        arguments: [
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
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 379,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            LinkagePath {
                                data: Method,
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 379,
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
                                    value: 380,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            F32(
                                NotNan(
                                    5.5,
                                ),
                            ),
                        ),
                        arguments: [],
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 382,
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
                                        value: 381,
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
                                    value: 384,
                                },
                            ),
                        ),
                        opn: ValOpn::TypeVariant(
                            TypeVariantPath {
                                parent_ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
                                ident: `Yes`,
                            },
                        ),
                        arguments: [],
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::ValItem(
                            FugitivePath(`mnist_classifier::major::major_concave_components`, `Val`),
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
                        opn: ValOpn::NewList,
                        arguments: [],
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 388,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            LinkagePath {
                                data: Item {
                                    path: MajorItem(
                                        Fugitive(
                                            FugitivePath(
                                                Id {
                                                    value: 25,
                                                },
                                            ),
                                        ),
                                    ),
                                    template_arguments: [],
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
                                    value: 388,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            LinkagePath {
                                data: Item {
                                    path: MajorItem(
                                        Fugitive(
                                            FugitivePath(
                                                Id {
                                                    value: 25,
                                                },
                                            ),
                                        ),
                                    ),
                                    template_arguments: [],
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
                                        value: 388,
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
                                    value: 389,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            LinkagePath {
                                data: Field,
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 390,
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
                                    value: 388,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            LinkagePath {
                                data: Item {
                                    path: MajorItem(
                                        Fugitive(
                                            FugitivePath(
                                                Id {
                                                    value: 25,
                                                },
                                            ),
                                        ),
                                    ),
                                    template_arguments: [],
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
                                        value: 388,
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
                                    value: 391,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            LinkagePath {
                                data: Field,
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 390,
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
                                    value: 388,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            LinkagePath {
                                data: Item {
                                    path: MajorItem(
                                        Fugitive(
                                            FugitivePath(
                                                Id {
                                                    value: 25,
                                                },
                                            ),
                                        ),
                                    ),
                                    template_arguments: [],
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
                                        value: 388,
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
                                    value: 392,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            LinkagePath {
                                data: Field,
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 390,
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
                            I32(
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
                                    value: 394,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            LinkagePath {
                                data: Item {
                                    path: MajorItem(
                                        Fugitive(
                                            FugitivePath(
                                                Id {
                                                    value: 78,
                                                },
                                            ),
                                        ),
                                    ),
                                    template_arguments: [],
                                },
                            },
                        ),
                        arguments: [
                            Variadic(
                                [
                                    ValRepr(
                                        Id {
                                            value: 391,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 392,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 393,
                                        },
                                    ),
                                ],
                            ),
                            Keyed(
                                Ident(
                                    Coword(
                                        Id {
                                            value: 453,
                                        },
                                    ),
                                ),
                                Some(
                                    ValRepr(
                                        Id {
                                            value: 394,
                                        },
                                    ),
                                ),
                            ),
                        ],
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 395,
                                },
                            ),
                        ),
                        opn: ValOpn::Suffix(
                            Unveil,
                        ),
                        arguments: [
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
                                    value: 388,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            LinkagePath {
                                data: Item {
                                    path: MajorItem(
                                        Fugitive(
                                            FugitivePath(
                                                Id {
                                                    value: 25,
                                                },
                                            ),
                                        ),
                                    ),
                                    template_arguments: [],
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
                                        value: 388,
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
                                    value: 396,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            LinkagePath {
                                data: Field,
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 390,
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
                                    value: 397,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            F32(
                                NotNan(
                                    3.0,
                                ),
                            ),
                        ),
                        arguments: [],
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 398,
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
                                        value: 397,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 398,
                                    },
                                ),
                            ),
                        ],
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::ValItem(
                            FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                        ),
                        arguments: [],
                        caching_class: ValItem,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 400,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            LinkagePath {
                                data: Field,
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
                                    value: 401,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            LinkagePath {
                                data: Field,
                            },
                        ),
                        arguments: [
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
                                    value: 402,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            USize(
                                TermUSizeLiteral(
                                    Id {
                                        value: 1,
                                    },
                                ),
                            ),
                        ),
                        arguments: [],
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 403,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            LinkagePath {
                                data: Index,
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 402,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 403,
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
                        opn: ValOpn::Be,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 404,
                                    },
                                ),
                            ),
                        ],
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::ValItem(
                            FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                        ),
                        arguments: [],
                        caching_class: ValItem,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 406,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            LinkagePath {
                                data: Field,
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
                                    value: 407,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            LinkagePath {
                                data: Field,
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 407,
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
                                    value: 408,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            USize(
                                TermUSizeLiteral(
                                    Id {
                                        value: 2,
                                    },
                                ),
                            ),
                        ),
                        arguments: [],
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 409,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            LinkagePath {
                                data: Index,
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 408,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 409,
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
                                    value: 410,
                                },
                            ),
                        ),
                        opn: ValOpn::Be,
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
                        val_domain_repr: Omni,
                        opn: ValOpn::ValItem(
                            FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                        ),
                        arguments: [],
                        caching_class: ValItem,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 412,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            LinkagePath {
                                data: Field,
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
                                    value: 413,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            LinkagePath {
                                data: Field,
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 413,
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
                                    value: 414,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            USize(
                                TermUSizeLiteral(
                                    Id {
                                        value: 2,
                                    },
                                ),
                            ),
                        ),
                        arguments: [],
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 415,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            LinkagePath {
                                data: Index,
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 414,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 415,
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
                                    value: 415,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            LinkagePath {
                                data: Index,
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 414,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 415,
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
                                    value: 416,
                                },
                            ),
                        ),
                        opn: ValOpn::Suffix(
                            Unwrap,
                        ),
                        arguments: [
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
                                    value: 418,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            LinkagePath {
                                data: Field,
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 418,
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
                                    value: 419,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            LinkagePath {
                                data: Method,
                            },
                        ),
                        arguments: [
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
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 415,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            LinkagePath {
                                data: Index,
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 414,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 415,
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
                                    value: 420,
                                },
                            ),
                        ),
                        opn: ValOpn::Suffix(
                            Unwrap,
                        ),
                        arguments: [
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
                        opn: ValOpn::Linkage(
                            LinkagePath {
                                data: Field,
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 421,
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
                                    value: 422,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            LinkagePath {
                                data: Method,
                            },
                        ),
                        arguments: [
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
                                    value: 423,
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
                                        value: 420,
                                    },
                                ),
                            ),
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
                        val_domain_repr: Omni,
                        opn: ValOpn::ValItem(
                            FugitivePath(`mnist_classifier::major::major_line_segment_sketch`, `Val`),
                        ),
                        arguments: [],
                        caching_class: ValItem,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 424,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            LinkagePath {
                                data: Field,
                            },
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
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 426,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            LinkagePath {
                                data: Method,
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 426,
                                    },
                                ),
                            ),
                        ],
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::ValItem(
                            FugitivePath(`mnist_classifier::major::major_line_segment_sketch`, `Val`),
                        ),
                        arguments: [],
                        caching_class: ValItem,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 427,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            LinkagePath {
                                data: Field,
                            },
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
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 428,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            LinkagePath {
                                data: Method,
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 428,
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
                                    value: 429,
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
                                        value: 427,
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
                                    value: 423,
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
                                        value: 420,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 423,
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
                                    value: 429,
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
                                        value: 427,
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
                        caching_class: Variable,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 430,
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
                                        value: 425,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 431,
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
                                    value: 430,
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
                                        value: 425,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 431,
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
                                    value: 432,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            F32(
                                NotNan(
                                    0.4,
                                ),
                            ),
                        ),
                        arguments: [],
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 434,
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
                                        value: 433,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 434,
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
                                    value: 388,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            LinkagePath {
                                data: Item {
                                    path: MajorItem(
                                        Fugitive(
                                            FugitivePath(
                                                Id {
                                                    value: 25,
                                                },
                                            ),
                                        ),
                                    ),
                                    template_arguments: [],
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
                                        value: 388,
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
                                    value: 436,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            LinkagePath {
                                data: Field,
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 390,
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
                                    value: 437,
                                },
                            ),
                        ),
                        opn: ValOpn::TypeVariant(
                            TypeVariantPath {
                                parent_ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
                                ident: `Yes`,
                            },
                        ),
                        arguments: [],
                        caching_class: Stmt,
                    },
                ],
                hir_lazy_stmt_val_repr_map: [
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 363,
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
                        ],
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 368,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 368,
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
                                    value: 373,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 373,
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
                                    value: 383,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 383,
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
                                    value: 385,
                                },
                            ),
                        ),
                        opn: ValOpn::Return,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 385,
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
                                    value: 354,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
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
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 355,
                                },
                            ),
                        ),
                        opn: ValOpn::Branches,
                        arguments: [
                            Branch {
                                condition: Some(
                                    ValRepr(
                                        Id {
                                            value: 359,
                                        },
                                    ),
                                ),
                                stmts: [
                                    ValRepr(
                                        Id {
                                            value: 364,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 369,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 374,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 384,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 386,
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
                                    value: 399,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 399,
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
                                    value: 405,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
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
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 411,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 411,
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
                                        value: 435,
                                    },
                                ),
                            ),
                        ],
                        caching_class: Stmt,
                    },
                ],
                root_hir_lazy_stmt_val_reprs: [
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 354,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
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
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 355,
                                },
                            ),
                        ),
                        opn: ValOpn::Branches,
                        arguments: [
                            Branch {
                                condition: Some(
                                    ValRepr(
                                        Id {
                                            value: 359,
                                        },
                                    ),
                                ),
                                stmts: [
                                    ValRepr(
                                        Id {
                                            value: 364,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 369,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 374,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 384,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 386,
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
                                    value: 395,
                                },
                            ),
                        ),
                        opn: ValOpn::Suffix(
                            Unveil,
                        ),
                        arguments: [
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
                                    value: 399,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 399,
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
                                    value: 405,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
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
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 411,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 411,
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
                                        value: 435,
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
                                    value: 437,
                                },
                            ),
                        ),
                        opn: ValOpn::TypeVariant(
                            TypeVariantPath {
                                parent_ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
                                ident: `Yes`,
                            },
                        ),
                        arguments: [],
                        caching_class: Stmt,
                    },
                ],
            },
        ),
    ),
]