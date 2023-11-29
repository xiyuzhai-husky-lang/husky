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
                            FugitivePath(`mnist_classifier::digits::six::six_match`, `Val`),
                        ),
                        arguments: [],
                        caching_class: ValItem,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 234,
                                },
                            ),
                        ),
                        opn: ValOpn::LinkageImpl(
                            Linkage {
                                data: LinkageData::PropsStructField,
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
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 235,
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
                                    value: 236,
                                },
                            ),
                        ),
                        opn: ValOpn::LinkageImpl(
                            Linkage {
                                data: LinkageData::Index,
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
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 236,
                                },
                            ),
                        ),
                        opn: ValOpn::LinkageImpl(
                            Linkage {
                                data: LinkageData::Index,
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
                        caching_class: Variable,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
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
                                    value: 240,
                                },
                            ),
                        ),
                        opn: ValOpn::LinkageImpl(
                            Linkage {
                                data: LinkageData::MemoizedField,
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
                                    value: 241,
                                },
                            ),
                        ),
                        opn: ValOpn::LinkageImpl(
                            Linkage {
                                data: LinkageData::MemoizedField,
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
                                    value: 243,
                                },
                            ),
                        ),
                        opn: ValOpn::LinkageImpl(
                            Linkage {
                                data: LinkageData::MemoizedField,
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
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 240,
                                },
                            ),
                        ),
                        opn: ValOpn::LinkageImpl(
                            Linkage {
                                data: LinkageData::MemoizedField,
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
                                    value: 245,
                                },
                            ),
                        ),
                        opn: ValOpn::LinkageImpl(
                            Linkage {
                                data: LinkageData::PropsStructField,
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
                                    value: 247,
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
                                    value: 248,
                                },
                            ),
                        ),
                        opn: ValOpn::LinkageImpl(
                            Linkage {
                                data: LinkageData::Index,
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
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
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
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::ValItem(
                            FugitivePath(`mnist_classifier::digits::six::six_match`, `Val`),
                        ),
                        arguments: [],
                        caching_class: ValItem,
                    },
                    ValRepr {
                        val_domain_repr: ConditionSatisfied(
                            ValRepr(
                                Id {
                                    value: 250,
                                },
                            ),
                        ),
                        opn: ValOpn::LinkageImpl(
                            Linkage {
                                data: LinkageData::MemoizedField,
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
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::ValItem(
                            FugitivePath(`mnist_classifier::digits::six::six_match_refined1`, `Val`),
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
                        opn: ValOpn::LinkageImpl(
                            Linkage {
                                data: LinkageData::MemoizedField,
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
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 252,
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
                                    value: 253,
                                },
                            ),
                        ),
                        opn: ValOpn::LinkageImpl(
                            Linkage {
                                data: LinkageData::PathLeading {
                                    path: Fugitive(
                                        FugitivePath(
                                            ItemPathId(
                                                Id {
                                                    value: 335,
                                                },
                                            ),
                                        ),
                                    ),
                                    instantiation: LinkageInstantiation {
                                        symbol_resolutions: [],
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
                                Some(
                                    ValRepr(
                                        Id {
                                            value: 253,
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
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::ValItem(
                            FugitivePath(`mnist_classifier::digits::six::six_match_refined1`, `Val`),
                        ),
                        arguments: [],
                        caching_class: ValItem,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 255,
                                },
                            ),
                        ),
                        opn: ValOpn::LinkageImpl(
                            Linkage {
                                data: LinkageData::PropsStructField,
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
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 256,
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
                                    value: 257,
                                },
                            ),
                        ),
                        opn: ValOpn::LinkageImpl(
                            Linkage {
                                data: LinkageData::Index,
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
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 257,
                                },
                            ),
                        ),
                        opn: ValOpn::LinkageImpl(
                            Linkage {
                                data: LinkageData::Index,
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
                        caching_class: Variable,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
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
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 260,
                                },
                            ),
                        ),
                        opn: ValOpn::LinkageImpl(
                            Linkage {
                                data: LinkageData::Method,
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
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 260,
                                },
                            ),
                        ),
                        opn: ValOpn::LinkageImpl(
                            Linkage {
                                data: LinkageData::Method,
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
                        caching_class: Variable,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 261,
                                },
                            ),
                        ),
                        opn: ValOpn::LinkageImpl(
                            Linkage {
                                data: LinkageData::PropsStructField,
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
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 236,
                                },
                            ),
                        ),
                        opn: ValOpn::LinkageImpl(
                            Linkage {
                                data: LinkageData::Index,
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
                        caching_class: Variable,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
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
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 265,
                                },
                            ),
                        ),
                        opn: ValOpn::LinkageImpl(
                            Linkage {
                                data: LinkageData::Method,
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
                        opn: ValOpn::LinkageImpl(
                            Linkage {
                                data: LinkageData::PropsStructField,
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
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::ValItem(
                            FugitivePath(`mnist_classifier::digits::six::six_match_refined1`, `Val`),
                        ),
                        arguments: [],
                        caching_class: ValItem,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 267,
                                },
                            ),
                        ),
                        opn: ValOpn::LinkageImpl(
                            Linkage {
                                data: LinkageData::PropsStructField,
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
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::ValItem(
                            FugitivePath(`mnist_classifier::digits::six::six_match_refined1`, `Val`),
                        ),
                        arguments: [],
                        caching_class: ValItem,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 269,
                                },
                            ),
                        ),
                        opn: ValOpn::LinkageImpl(
                            Linkage {
                                data: LinkageData::MemoizedField,
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
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 271,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            TermLiteral::F32(
                                NotNan(
                                    1.8,
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
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 257,
                                },
                            ),
                        ),
                        opn: ValOpn::LinkageImpl(
                            Linkage {
                                data: LinkageData::Index,
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
                        caching_class: Variable,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
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
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 261,
                                },
                            ),
                        ),
                        opn: ValOpn::LinkageImpl(
                            Linkage {
                                data: LinkageData::PropsStructField,
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
                        caching_class: Variable,
                    },
                    ValRepr {
                        val_domain_repr: ConditionSatisfied(
                            ValRepr(
                                Id {
                                    value: 275,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            TermLiteral::F32(
                                NotNan(
                                    2.5,
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
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
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
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::ValItem(
                            FugitivePath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Val`),
                        ),
                        arguments: [],
                        caching_class: ValItem,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 279,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            TermLiteral::F32(
                                NotNan(
                                    30.0,
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
                        opn: ValOpn::LinkageImpl(
                            Linkage {
                                data: LinkageData::PropsStructField,
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
                        caching_class: Variable,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 282,
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
                                    value: 283,
                                },
                            ),
                        ),
                        opn: ValOpn::LinkageImpl(
                            Linkage {
                                data: LinkageData::PathLeading {
                                    path: Fugitive(
                                        FugitivePath(
                                            ItemPathId(
                                                Id {
                                                    value: 335,
                                                },
                                            ),
                                        ),
                                    ),
                                    instantiation: LinkageInstantiation {
                                        symbol_resolutions: [],
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
                                Some(
                                    ValRepr(
                                        Id {
                                            value: 283,
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
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 285,
                                },
                            ),
                        ),
                        opn: ValOpn::TypeVariant(
                            TypeVariantPath(
                                ItemPathId {
                                    data: ItemPathData::TypeVariant(
                                        TypeVariantPathData {
                                            parent_ty_path: TypePath(
                                                ItemPathId(
                                                    Id {
                                                        value: 328,
                                                    },
                                                ),
                                            ),
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 450,
                                                    },
                                                ),
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
                                    value: 288,
                                },
                            ),
                        ),
                        opn: ValOpn::LinkageImpl(
                            Linkage {
                                data: LinkageData::MemoizedField,
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
                                    value: 236,
                                },
                            ),
                        ),
                        opn: ValOpn::LinkageImpl(
                            Linkage {
                                data: LinkageData::Index,
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
                        caching_class: Variable,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
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
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 291,
                                },
                            ),
                        ),
                        opn: ValOpn::LinkageImpl(
                            Linkage {
                                data: LinkageData::Method,
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
                        opn: ValOpn::LinkageImpl(
                            Linkage {
                                data: LinkageData::Method,
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
                        opn: ValOpn::LinkageImpl(
                            Linkage {
                                data: LinkageData::PropsStructField,
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
                        caching_class: Variable,
                    },
                    ValRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::ValItem(
                            FugitivePath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Val`),
                        ),
                        arguments: [],
                        caching_class: ValItem,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
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
                        caching_class: Variable,
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
                                    value: 293,
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
                                    value: 295,
                                },
                            ),
                        ),
                        opn: ValOpn::LinkageImpl(
                            Linkage {
                                data: LinkageData::Method,
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
                                    value: 297,
                                },
                            ),
                        ),
                        opn: ValOpn::LinkageImpl(
                            Linkage {
                                data: LinkageData::PathLeading {
                                    path: Fugitive(
                                        FugitivePath(
                                            ItemPathId(
                                                Id {
                                                    value: 335,
                                                },
                                            ),
                                        ),
                                    ),
                                    instantiation: LinkageInstantiation {
                                        symbol_resolutions: [],
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
                                Some(
                                    ValRepr(
                                        Id {
                                            value: 297,
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
                        opn: ValOpn::LinkageImpl(
                            Linkage {
                                data: LinkageData::Method,
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
                        caching_class: Variable,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 299,
                                },
                            ),
                        ),
                        opn: ValOpn::LinkageImpl(
                            Linkage {
                                data: LinkageData::PropsStructField,
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
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 300,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            TermLiteral::F32(
                                NotNan(
                                    0.7,
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
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: ConditionSatisfied(
                            ValRepr(
                                Id {
                                    value: 302,
                                },
                            ),
                        ),
                        opn: ValOpn::TypeVariant(
                            TypeVariantPath(
                                ItemPathId {
                                    data: ItemPathData::TypeVariant(
                                        TypeVariantPathData {
                                            parent_ty_path: TypePath(
                                                ItemPathId(
                                                    Id {
                                                        value: 328,
                                                    },
                                                ),
                                            ),
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 450,
                                                    },
                                                ),
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
                                    value: 305,
                                },
                            ),
                        ),
                        opn: ValOpn::TypeVariant(
                            TypeVariantPath(
                                ItemPathId {
                                    data: ItemPathData::TypeVariant(
                                        TypeVariantPathData {
                                            parent_ty_path: TypePath(
                                                ItemPathId(
                                                    Id {
                                                        value: 328,
                                                    },
                                                ),
                                            ),
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 450,
                                                    },
                                                ),
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
                        opn: ValOpn::ValItem(
                            FugitivePath(`mnist_classifier::digits::six::six_match`, `Val`),
                        ),
                        arguments: [],
                        caching_class: ValItem,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 308,
                                },
                            ),
                        ),
                        opn: ValOpn::LinkageImpl(
                            Linkage {
                                data: LinkageData::MemoizedField,
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
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 309,
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
                                    value: 310,
                                },
                            ),
                        ),
                        opn: ValOpn::LinkageImpl(
                            Linkage {
                                data: LinkageData::PathLeading {
                                    path: Fugitive(
                                        FugitivePath(
                                            ItemPathId(
                                                Id {
                                                    value: 335,
                                                },
                                            ),
                                        ),
                                    ),
                                    instantiation: LinkageInstantiation {
                                        symbol_resolutions: [],
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
                                Some(
                                    ValRepr(
                                        Id {
                                            value: 310,
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
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::ValItem(
                            FugitivePath(`mnist_classifier::digits::six::six_match`, `Val`),
                        ),
                        arguments: [],
                        caching_class: ValItem,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 312,
                                },
                            ),
                        ),
                        opn: ValOpn::LinkageImpl(
                            Linkage {
                                data: LinkageData::MemoizedField,
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
                                NotNan(
                                    1.8,
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
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::ValItem(
                            FugitivePath(`mnist_classifier::digits::six::six_match`, `Val`),
                        ),
                        arguments: [],
                        caching_class: ValItem,
                    },
                    ValRepr {
                        val_domain_repr: ConditionSatisfied(
                            ValRepr(
                                Id {
                                    value: 315,
                                },
                            ),
                        ),
                        opn: ValOpn::LinkageImpl(
                            Linkage {
                                data: LinkageData::MemoizedField,
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
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 316,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            TermLiteral::F32(
                                NotNan(
                                    1.8,
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
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 240,
                                },
                            ),
                        ),
                        opn: ValOpn::LinkageImpl(
                            Linkage {
                                data: LinkageData::MemoizedField,
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
                                    value: 320,
                                },
                            ),
                        ),
                        opn: ValOpn::LinkageImpl(
                            Linkage {
                                data: LinkageData::PropsStructField,
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
                                    value: 321,
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
                                    value: 322,
                                },
                            ),
                        ),
                        opn: ValOpn::LinkageImpl(
                            Linkage {
                                data: LinkageData::Index,
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
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
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
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 324,
                                },
                            ),
                        ),
                        opn: ValOpn::LinkageImpl(
                            Linkage {
                                data: LinkageData::MemoizedField,
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
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 325,
                                },
                            ),
                        ),
                        opn: ValOpn::LinkageImpl(
                            Linkage {
                                data: LinkageData::Method,
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
                                NotNan(
                                    0.75,
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
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 240,
                                },
                            ),
                        ),
                        opn: ValOpn::LinkageImpl(
                            Linkage {
                                data: LinkageData::MemoizedField,
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
                                    value: 329,
                                },
                            ),
                        ),
                        opn: ValOpn::LinkageImpl(
                            Linkage {
                                data: LinkageData::PropsStructField,
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
                                    value: 330,
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
                                    value: 331,
                                },
                            ),
                        ),
                        opn: ValOpn::LinkageImpl(
                            Linkage {
                                data: LinkageData::Index,
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
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
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
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 240,
                                },
                            ),
                        ),
                        opn: ValOpn::LinkageImpl(
                            Linkage {
                                data: LinkageData::MemoizedField,
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
                                    value: 333,
                                },
                            ),
                        ),
                        opn: ValOpn::LinkageImpl(
                            Linkage {
                                data: LinkageData::PropsStructField,
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
                                    value: 334,
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
                                    value: 335,
                                },
                            ),
                        ),
                        opn: ValOpn::LinkageImpl(
                            Linkage {
                                data: LinkageData::Index,
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
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
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
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 337,
                                },
                            ),
                        ),
                        opn: ValOpn::LinkageImpl(
                            Linkage {
                                data: LinkageData::MemoizedField,
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
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 338,
                                },
                            ),
                        ),
                        opn: ValOpn::LinkageImpl(
                            Linkage {
                                data: LinkageData::Method,
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
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 339,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            TermLiteral::F32(
                                NotNan(
                                    0.75,
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
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
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
                        caching_class: Variable,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 343,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            TermLiteral::F32(
                                NotNan(
                                    15.0,
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
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 346,
                                },
                            ),
                        ),
                        opn: ValOpn::TypeVariant(
                            TypeVariantPath(
                                ItemPathId {
                                    data: ItemPathData::TypeVariant(
                                        TypeVariantPathData {
                                            parent_ty_path: TypePath(
                                                ItemPathId(
                                                    Id {
                                                        value: 328,
                                                    },
                                                ),
                                            ),
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 450,
                                                    },
                                                ),
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
                        val_domain_repr: StmtNotReturned(
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
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
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
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
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
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
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
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
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
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
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
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
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
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
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
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
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
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
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
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
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
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
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
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
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
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
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
                        caching_class: Stmt,
                    },
                ],
                root_hir_lazy_stmt_val_reprs: [
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
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
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
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
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
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
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
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
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
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
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
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
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
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
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 346,
                                },
                            ),
                        ),
                        opn: ValOpn::TypeVariant(
                            TypeVariantPath(
                                ItemPathId {
                                    data: ItemPathData::TypeVariant(
                                        TypeVariantPathData {
                                            parent_ty_path: TypePath(
                                                ItemPathId(
                                                    Id {
                                                        value: 328,
                                                    },
                                                ),
                                            ),
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 450,
                                                    },
                                                ),
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