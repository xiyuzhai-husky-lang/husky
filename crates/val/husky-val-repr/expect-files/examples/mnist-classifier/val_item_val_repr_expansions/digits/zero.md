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
                                    value: 359,
                                },
                            ),
                        ),
                        None,
                        Some(
                            ValRepr(
                                Id {
                                    value: 379,
                                },
                            ),
                        ),
                        Some(
                            ValRepr(
                                Id {
                                    value: 388,
                                },
                            ),
                        ),
                        None,
                        None,
                        Some(
                            ValRepr(
                                Id {
                                    value: 415,
                                },
                            ),
                        ),
                        Some(
                            ValRepr(
                                Id {
                                    value: 423,
                                },
                            ),
                        ),
                        Some(
                            ValRepr(
                                Id {
                                    value: 429,
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
                                    value: 436,
                                },
                            ),
                        ),
                    ],
                },
                hir_lazy_expr_val_repr_map: [
                    Val {
                        domain_repr: Omni,
                        opn: ValOpn::ValItem(
                            FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                        ),
                        arguments: [],
                    },
                    Val {
                        domain_repr: Omni,
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
                    },
                    Val {
                        domain_repr: Omni,
                        opn: ValOpn::ValItem(
                            FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                        ),
                        arguments: [],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 234,
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
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 354,
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
                                        value: 354,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 355,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            I32(
                                1,
                            ),
                        ),
                        arguments: [],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 356,
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
                                        value: 355,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 356,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: Omni,
                        opn: ValOpn::ValItem(
                            FugitivePath(`mnist_classifier::digits::zero::open_one_match`, `Val`),
                        ),
                        arguments: [],
                    },
                    Val {
                        domain_repr: ConditionSatisfied(
                            ValRepr(
                                Id {
                                    value: 357,
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
                    },
                    Val {
                        domain_repr: ConditionSatisfied(
                            ValRepr(
                                Id {
                                    value: 357,
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
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 358,
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
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 360,
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
                                        value: 359,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 360,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: Omni,
                        opn: ValOpn::ValItem(
                            FugitivePath(`mnist_classifier::digits::zero::open_one_match`, `Val`),
                        ),
                        arguments: [],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 362,
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
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 363,
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
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 364,
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
                                        value: 363,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 364,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 365,
                                },
                            ),
                        ),
                        opn: ValOpn::Be,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 365,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: Omni,
                        opn: ValOpn::ValItem(
                            FugitivePath(`mnist_classifier::major::connected_components`, `Val`),
                        ),
                        arguments: [],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 367,
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
                                        value: 368,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 369,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            I32(
                                1,
                            ),
                        ),
                        arguments: [],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 370,
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
                                        value: 369,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 370,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: Omni,
                        opn: ValOpn::ValItem(
                            FugitivePath(`mnist_classifier::digits::zero::open_one_match`, `Val`),
                        ),
                        arguments: [],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 372,
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
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 373,
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
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 374,
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
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 375,
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
                                        value: 375,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 376,
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
                                        value: 376,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 377,
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
                                        value: 377,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 377,
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
                                        value: 377,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 378,
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
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 380,
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
                                        value: 379,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 380,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 382,
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
                    },
                    Val {
                        domain_repr: Omni,
                        opn: ValOpn::ValItem(
                            FugitivePath(`mnist_classifier::major::major_concave_components`, `Val`),
                        ),
                        arguments: [],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 385,
                                },
                            ),
                        ),
                        opn: ValOpn::NewList,
                        arguments: [],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 386,
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
                                        value: 386,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 386,
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
                                        value: 386,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 387,
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
                                        value: 388,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 386,
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
                                        value: 386,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
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
                                        value: 388,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 386,
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
                                        value: 386,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 390,
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
                                        value: 388,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 391,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            I32(
                                5,
                            ),
                        ),
                        arguments: [],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 392,
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
                                            value: 389,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 390,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 391,
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
                                ValRepr(
                                    Id {
                                        value: 392,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 393,
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
                                        value: 393,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 386,
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
                                        value: 386,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 394,
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
                                        value: 388,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 395,
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
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 396,
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
                                        value: 395,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 396,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: Omni,
                        opn: ValOpn::ValItem(
                            FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                        ),
                        arguments: [],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 398,
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
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 399,
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
                                        value: 399,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 400,
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
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 401,
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
                                        value: 400,
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
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 402,
                                },
                            ),
                        ),
                        opn: ValOpn::Be,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 402,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: Omni,
                        opn: ValOpn::ValItem(
                            FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                        ),
                        arguments: [],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 404,
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
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 405,
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
                                        value: 405,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 406,
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
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 407,
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
                                        value: 406,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 407,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 408,
                                },
                            ),
                        ),
                        opn: ValOpn::Be,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 408,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: Omni,
                        opn: ValOpn::ValItem(
                            FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                        ),
                        arguments: [],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 410,
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
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 411,
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
                                        value: 411,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 412,
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
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 413,
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
                                        value: 412,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 413,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 413,
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
                                        value: 412,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 413,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 414,
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
                                        value: 415,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 416,
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
                                        value: 416,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 417,
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
                                        value: 417,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 413,
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
                                        value: 412,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 413,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 418,
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
                                        value: 415,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 419,
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
                                        value: 419,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 420,
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
                                        value: 420,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 421,
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
                                        value: 418,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 421,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: Omni,
                        opn: ValOpn::ValItem(
                            FugitivePath(`mnist_classifier::major::major_line_segment_sketch`, `Val`),
                        ),
                        arguments: [],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 422,
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
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 424,
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
                                        value: 424,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: Omni,
                        opn: ValOpn::ValItem(
                            FugitivePath(`mnist_classifier::major::major_line_segment_sketch`, `Val`),
                        ),
                        arguments: [],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 425,
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
                    },
                    Val {
                        domain_repr: StmtNotReturned(
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
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 427,
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
                                        value: 425,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 427,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 421,
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
                                        value: 418,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 421,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 427,
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
                                        value: 425,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 427,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 428,
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
                                        value: 423,
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
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 428,
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
                                        value: 423,
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
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 430,
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
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 432,
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
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 386,
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
                                        value: 386,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 434,
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
                                        value: 388,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 435,
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
                    },
                ],
                hir_lazy_stmt_val_repr_map: [
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 361,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 361,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 366,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 366,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 371,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 371,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
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
                                        value: 381,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 383,
                                },
                            ),
                        ),
                        opn: ValOpn::Return,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 383,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 233,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 233,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 234,
                                },
                            ),
                        ),
                        opn: ValOpn::Branches,
                        arguments: [
                            Branch {
                                condition: Some(
                                    ValRepr(
                                        Id {
                                            value: 357,
                                        },
                                    ),
                                ),
                                stmts: [
                                    ValRepr(
                                        Id {
                                            value: 362,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 367,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 372,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 382,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 384,
                                        },
                                    ),
                                ],
                            },
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 397,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 397,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 403,
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
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 409,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 409,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 433,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 433,
                                    },
                                ),
                            ),
                        ],
                    },
                ],
                root_hir_lazy_stmt_val_reprs: [
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 233,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 233,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 234,
                                },
                            ),
                        ),
                        opn: ValOpn::Branches,
                        arguments: [
                            Branch {
                                condition: Some(
                                    ValRepr(
                                        Id {
                                            value: 357,
                                        },
                                    ),
                                ),
                                stmts: [
                                    ValRepr(
                                        Id {
                                            value: 362,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 367,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 372,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 382,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 384,
                                        },
                                    ),
                                ],
                            },
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 393,
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
                                        value: 393,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 397,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 397,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 403,
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
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 409,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 409,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 433,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 433,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 435,
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
                    },
                ],
            },
        ),
    ),
]