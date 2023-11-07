[
    HirDecl::MajorItem(
        MajorItemHirDecl::Fugitive(
            FugitiveHirDecl::Val(
                ValFugitiveHirDecl {
                    path: FugitivePath(`mnist_classifier::digits::zero::open_one_match`, `Val`),
                    hir_expr_region: Eager(
                        HirEagerExprRegion(
                            Id {
                                value: 1,
                            },
                        ),
                    ),
                },
            ),
        ),
    ),
    HirDecl::MajorItem(
        MajorItemHirDecl::Fugitive(
            FugitiveHirDecl::FunctionFn(
                FunctionFnFugitiveHirDecl {
                    path: FugitivePath(`mnist_classifier::digits::zero::almost_closed`, `FunctionFn`),
                    template_parameters: HirTemplateParameters(
                        [],
                    ),
                    parenate_parameters: HirEagerParenateParameters(
                        [
                            Ordinary {
                                pattern_expr_idx: 1,
                                ty: PathLeading(
                                    HirTypePathLeading(
                                        Id {
                                            value: 42,
                                        },
                                    ),
                                ),
                            },
                        ],
                    ),
                    return_ty: HirType::PathLeading(
                        HirTypePathLeading {
                            ty_path: TypePath(`core::option::Option`, `Enum`),
                            template_arguments: [
                                Type(
                                    PathLeading(
                                        HirTypePathLeading(
                                            Id {
                                                value: 14,
                                            },
                                        ),
                                    ),
                                ),
                            ],
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion {
                        hir_eager_expr_arena: Arena {
                            data: [],
                        },
                        hir_eager_stmt_arena: Arena {
                            data: [],
                        },
                        hir_eager_pattern_expr_arena: Arena {
                            data: [
                                Ident {
                                    symbol_modifier: None,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 286,
                                            },
                                        ),
                                    ),
                                },
                            ],
                        },
                        hir_eager_variable_region: HirEagerVariableRegion {
                            arena: Arena {
                                data: [
                                    HirEagerVariable {
                                        name: Ident(
                                            Ident(
                                                Coword(
                                                    Id {
                                                        value: 286,
                                                    },
                                                ),
                                            ),
                                        ),
                                        data: ParenateParameter,
                                    },
                                ],
                            },
                        },
                    },
                },
            ),
        ),
    ),
    HirDecl::MajorItem(
        MajorItemHirDecl::Fugitive(
            FugitiveHirDecl::Val(
                ValFugitiveHirDecl {
                    path: FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                    hir_expr_region: Eager(
                        HirEagerExprRegion(
                            Id {
                                value: 1,
                            },
                        ),
                    ),
                },
            ),
        ),
    ),
]