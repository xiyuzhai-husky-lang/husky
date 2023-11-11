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
                [salsa id]: 1,
                hir_lazy_variable_val_repr_map: ArenaMap {
                    data: [
                        None,
                        Some(
                            ValRepr(
                                Id {
                                    value: 238,
                                },
                            ),
                        ),
                        None,
                        Some(
                            ValRepr(
                                Id {
                                    value: 242,
                                },
                            ),
                        ),
                        Some(
                            ValRepr(
                                Id {
                                    value: 246,
                                },
                            ),
                        ),
                        None,
                        Some(
                            ValRepr(
                                Id {
                                    value: 259,
                                },
                            ),
                        ),
                        Some(
                            ValRepr(
                                Id {
                                    value: 262,
                                },
                            ),
                        ),
                        Some(
                            ValRepr(
                                Id {
                                    value: 264,
                                },
                            ),
                        ),
                        Some(
                            ValRepr(
                                Id {
                                    value: 268,
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
                        None,
                        Some(
                            ValRepr(
                                Id {
                                    value: 294,
                                },
                            ),
                        ),
                        None,
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
                            FugitivePath(`mnist_classifier::digits::six::six_match`, `Val`),
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
                                        value: 230,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 235,
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
                                    value: 236,
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
                                        value: 235,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 236,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 236,
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
                                        value: 235,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 236,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 237,
                                },
                            ),
                        ),
                        opn: ValOpn::Be,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 238,
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
                                    value: 240,
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
                                    value: 241,
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
                                    value: 243,
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
                                    value: 244,
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
                                        value: 243,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 244,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 240,
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
                                    value: 245,
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
                                        value: 242,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 247,
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
                                    value: 248,
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
                                        value: 247,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 248,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 249,
                                },
                            ),
                        ),
                        opn: ValOpn::Be,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 249,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: Omni,
                        opn: ValOpn::ValItem(
                            FugitivePath(`mnist_classifier::digits::six::six_match`, `Val`),
                        ),
                        arguments: [],
                    },
                    Val {
                        domain_repr: ConditionSatisfied(
                            ValRepr(
                                Id {
                                    value: 250,
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
                                        value: 230,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: Omni,
                        opn: ValOpn::ValItem(
                            FugitivePath(`mnist_classifier::digits::six::six_match_refined1`, `Val`),
                        ),
                        arguments: [],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 251,
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
                                        value: 231,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 252,
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
                                    value: 253,
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
                                            value: 251,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 252,
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
                                        value: 253,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 254,
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
                                        value: 254,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: Omni,
                        opn: ValOpn::ValItem(
                            FugitivePath(`mnist_classifier::digits::six::six_match_refined1`, `Val`),
                        ),
                        arguments: [],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 255,
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
                                        value: 231,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 256,
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
                                    value: 257,
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
                                        value: 256,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 257,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 257,
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
                                        value: 256,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 257,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 258,
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
                                        value: 259,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 260,
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
                                        value: 260,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 260,
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
                                        value: 260,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 261,
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
                                        value: 262,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 236,
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
                                        value: 235,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 236,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 263,
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
                                        value: 238,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 265,
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
                                        value: 265,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 266,
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
                                        value: 266,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: Omni,
                        opn: ValOpn::ValItem(
                            FugitivePath(`mnist_classifier::digits::six::six_match_refined1`, `Val`),
                        ),
                        arguments: [],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 267,
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
                                        value: 231,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: Omni,
                        opn: ValOpn::ValItem(
                            FugitivePath(`mnist_classifier::digits::six::six_match_refined1`, `Val`),
                        ),
                        arguments: [],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 269,
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
                                        value: 231,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 271,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            F32(
                                NotNan(
                                    1.8,
                                ),
                            ),
                        ),
                        arguments: [],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 272,
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
                                        value: 271,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 272,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 257,
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
                                        value: 256,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 257,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 274,
                                },
                            ),
                        ),
                        opn: ValOpn::Be,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 259,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 261,
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
                                        value: 262,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: ConditionSatisfied(
                            ValRepr(
                                Id {
                                    value: 275,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            F32(
                                NotNan(
                                    2.5,
                                ),
                            ),
                        ),
                        arguments: [],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 276,
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
                                        value: 276,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 277,
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
                                        value: 264,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 277,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: Omni,
                        opn: ValOpn::ValItem(
                            FugitivePath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Val`),
                        ),
                        arguments: [],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 279,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            F32(
                                NotNan(
                                    30.0,
                                ),
                            ),
                        ),
                        arguments: [],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 280,
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
                                        value: 280,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 266,
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
                                        value: 266,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 282,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            I32(
                                15,
                            ),
                        ),
                        arguments: [],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 283,
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
                                            value: 268,
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
                                        value: 283,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 284,
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
                                        value: 284,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 285,
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
                            FugitivePath(`mnist_classifier::major::major_line_segment_sketch`, `Val`),
                        ),
                        arguments: [],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 288,
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
                                    value: 236,
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
                                        value: 235,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 236,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 290,
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
                                        value: 238,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 291,
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
                                        value: 291,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 292,
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
                                        value: 290,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 292,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 266,
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
                                        value: 266,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: Omni,
                        opn: ValOpn::ValItem(
                            FugitivePath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Val`),
                        ),
                        arguments: [],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 244,
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
                                        value: 243,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 244,
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
                                    value: 293,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            I32(
                                6,
                            ),
                        ),
                        arguments: [],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 295,
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
                                        value: 14,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 295,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 296,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            I32(
                                15,
                            ),
                        ),
                        arguments: [],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 297,
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
                                            value: 268,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 16,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 246,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 296,
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
                                        value: 297,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 298,
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
                                        value: 298,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 292,
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
                                        value: 290,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 292,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 299,
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
                                        value: 294,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 300,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            F32(
                                NotNan(
                                    0.7,
                                ),
                            ),
                        ),
                        arguments: [],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 301,
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
                                        value: 300,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 301,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: ConditionSatisfied(
                            ValRepr(
                                Id {
                                    value: 302,
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
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 305,
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
                            FugitivePath(`mnist_classifier::digits::six::six_match`, `Val`),
                        ),
                        arguments: [],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 308,
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
                                        value: 230,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 309,
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
                                    value: 310,
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
                                            value: 309,
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
                                        value: 310,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 311,
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
                                        value: 311,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: Omni,
                        opn: ValOpn::ValItem(
                            FugitivePath(`mnist_classifier::digits::six::six_match`, `Val`),
                        ),
                        arguments: [],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 312,
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
                                        value: 230,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 313,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            F32(
                                NotNan(
                                    1.8,
                                ),
                            ),
                        ),
                        arguments: [],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 314,
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
                                        value: 313,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 314,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: Omni,
                        opn: ValOpn::ValItem(
                            FugitivePath(`mnist_classifier::digits::six::six_match`, `Val`),
                        ),
                        arguments: [],
                    },
                    Val {
                        domain_repr: ConditionSatisfied(
                            ValRepr(
                                Id {
                                    value: 315,
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
                                        value: 230,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 316,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            F32(
                                NotNan(
                                    1.8,
                                ),
                            ),
                        ),
                        arguments: [],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 317,
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
                                        value: 316,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 317,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 240,
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
                                    value: 320,
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
                                        value: 242,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 321,
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
                                    value: 322,
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
                                        value: 321,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 322,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 323,
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
                                        value: 323,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 324,
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
                                        value: 324,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 325,
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
                                        value: 325,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 326,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            F32(
                                NotNan(
                                    0.75,
                                ),
                            ),
                        ),
                        arguments: [],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 327,
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
                                        value: 326,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 327,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 240,
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
                                    value: 329,
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
                                        value: 242,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 330,
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
                                    value: 331,
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
                                        value: 330,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 331,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 332,
                                },
                            ),
                        ),
                        opn: ValOpn::Be,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 332,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 240,
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
                        domain_repr: ConditionSatisfied(
                            ValRepr(
                                Id {
                                    value: 333,
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
                                        value: 242,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 334,
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
                                    value: 335,
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
                                        value: 334,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 335,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 336,
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
                                        value: 336,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 337,
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
                                        value: 337,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 338,
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
                                        value: 338,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 339,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            F32(
                                NotNan(
                                    0.75,
                                ),
                            ),
                        ),
                        arguments: [],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 340,
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
                                        value: 339,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 340,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 244,
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
                                        value: 243,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 244,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 343,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            F32(
                                NotNan(
                                    15.0,
                                ),
                            ),
                        ),
                        arguments: [],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 344,
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
                                        value: 246,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 344,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 346,
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
                                    value: 278,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 278,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 281,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 281,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 286,
                                },
                            ),
                        ),
                        opn: ValOpn::Return,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 286,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 303,
                                },
                            ),
                        ),
                        opn: ValOpn::Return,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 303,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 273,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 273,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 274,
                                },
                            ),
                        ),
                        opn: ValOpn::Branches,
                        arguments: [
                            Branch {
                                condition: Some(
                                    ValRepr(
                                        Id {
                                            value: 275,
                                        },
                                    ),
                                ),
                                stmts: [
                                    ValRepr(
                                        Id {
                                            value: 279,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 282,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 285,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 287,
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
                                    value: 299,
                                },
                            ),
                        ),
                        opn: ValOpn::Branches,
                        arguments: [
                            Branch {
                                condition: Some(
                                    ValRepr(
                                        Id {
                                            value: 302,
                                        },
                                    ),
                                ),
                                stmts: [
                                    ValRepr(
                                        Id {
                                            value: 304,
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
                                    value: 306,
                                },
                            ),
                        ),
                        opn: ValOpn::Return,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 306,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 318,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 318,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 341,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 341,
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
                                    value: 239,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 239,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 245,
                                },
                            ),
                        ),
                        opn: ValOpn::Branches,
                        arguments: [
                            Branch {
                                condition: Some(
                                    ValRepr(
                                        Id {
                                            value: 250,
                                        },
                                    ),
                                ),
                                stmts: [
                                    ValRepr(
                                        Id {
                                            value: 255,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 274,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 288,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 299,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 305,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 307,
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
                                    value: 312,
                                },
                            ),
                        ),
                        opn: ValOpn::Branches,
                        arguments: [
                            Branch {
                                condition: Some(
                                    ValRepr(
                                        Id {
                                            value: 315,
                                        },
                                    ),
                                ),
                                stmts: [
                                    ValRepr(
                                        Id {
                                            value: 319,
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
                                    value: 328,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 328,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 329,
                                },
                            ),
                        ),
                        opn: ValOpn::Branches,
                        arguments: [
                            Branch {
                                condition: Some(
                                    ValRepr(
                                        Id {
                                            value: 333,
                                        },
                                    ),
                                ),
                                stmts: [
                                    ValRepr(
                                        Id {
                                            value: 342,
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
                                    value: 345,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 345,
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
                                    value: 239,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 239,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 245,
                                },
                            ),
                        ),
                        opn: ValOpn::Branches,
                        arguments: [
                            Branch {
                                condition: Some(
                                    ValRepr(
                                        Id {
                                            value: 250,
                                        },
                                    ),
                                ),
                                stmts: [
                                    ValRepr(
                                        Id {
                                            value: 255,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 274,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 288,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 299,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 305,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 307,
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
                                    value: 311,
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
                                        value: 311,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 312,
                                },
                            ),
                        ),
                        opn: ValOpn::Branches,
                        arguments: [
                            Branch {
                                condition: Some(
                                    ValRepr(
                                        Id {
                                            value: 315,
                                        },
                                    ),
                                ),
                                stmts: [
                                    ValRepr(
                                        Id {
                                            value: 319,
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
                                    value: 328,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 328,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 329,
                                },
                            ),
                        ),
                        opn: ValOpn::Branches,
                        arguments: [
                            Branch {
                                condition: Some(
                                    ValRepr(
                                        Id {
                                            value: 333,
                                        },
                                    ),
                                ),
                                stmts: [
                                    ValRepr(
                                        Id {
                                            value: 342,
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
                                    value: 345,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 345,
                                    },
                                ),
                            ),
                        ],
                    },
                    Val {
                        domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 346,
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