[
    HirDecl::MajorItem(
        MajorItemHirDecl::Fugitive(
            FugitiveHirDecl::FunctionFn(
                FunctionFnFugitiveHirDecl {
                    path: FugitivePath(`mnist_classifier::line_segment_sketch::convexity::is_convex`, `FunctionFn`),
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
                                            value: 57,
                                        },
                                    ),
                                ),
                            },
                            Ordinary {
                                pattern_expr_idx: 2,
                                ty: PathLeading(
                                    HirTypePathLeading(
                                        Id {
                                            value: 5,
                                        },
                                    ),
                                ),
                            },
                        ],
                    ),
                    return_ty: HirType::PathLeading(
                        HirTypePathLeading {
                            ty_path: TypePath(`core::basic::bool`, `Extern`),
                            template_arguments: [],
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
                                                value: 220,
                                            },
                                        ),
                                    ),
                                },
                                Ident {
                                    symbol_modifier: None,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 421,
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
                                                        value: 220,
                                                    },
                                                ),
                                            ),
                                        ),
                                        data: ParenateParameter,
                                    },
                                    HirEagerVariable {
                                        name: Ident(
                                            Ident(
                                                Coword(
                                                    Id {
                                                        value: 421,
                                                    },
                                                ),
                                            ),
                                        ),
                                        data: ParenateParameter,
                                    },
                                ],
                            },
                            self_value_variable: None,
                        },
                    },
                },
            ),
        ),
    ),
]