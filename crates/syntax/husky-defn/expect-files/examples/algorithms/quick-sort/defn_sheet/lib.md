Ok(
    DefnSheet {
        defns: [
            Form(
                Function(
                    FunctionDefn {
                        path: FormPath(`quick_sort::quick_sort`, `Function`),
                        decl: FunctionDecl(
                            Id {
                                value: 1,
                            },
                        ),
                        expr_region: ExprRegion(
                            Id {
                                value: 55,
                            },
                        ),
                        body: Ok(
                            12,
                        ),
                    },
                ),
            ),
            Form(
                Function(
                    FunctionDefn {
                        path: FormPath(`quick_sort::quick_sort_aux`, `Function`),
                        decl: FunctionDecl(
                            Id {
                                value: 2,
                            },
                        ),
                        expr_region: ExprRegion(
                            Id {
                                value: 56,
                            },
                        ),
                        body: Ok(
                            22,
                        ),
                    },
                ),
            ),
            Form(
                Function(
                    FunctionDefn {
                        path: FormPath(`quick_sort::partition`, `Function`),
                        decl: FunctionDecl(
                            Id {
                                value: 3,
                            },
                        ),
                        expr_region: ExprRegion(
                            Id {
                                value: 57,
                            },
                        ),
                        body: Ok(
                            62,
                        ),
                    },
                ),
            ),
            Form(
                Feature(
                    FeatureDefn {
                        path: FormPath(`quick_sort::quick_sort_works_for_integers`, `Feature`),
                        decl: FeatureDecl(
                            Id {
                                value: 1,
                            },
                        ),
                        expr_region: ExprRegion(
                            Id {
                                value: 58,
                            },
                        ),
                        body: Ok(
                            29,
                        ),
                    },
                ),
            ),
            Form(
                Feature(
                    FeatureDefn {
                        path: FormPath(`quick_sort::quick_sort_works_for_strs`, `Feature`),
                        decl: FeatureDecl(
                            Id {
                                value: 2,
                            },
                        ),
                        expr_region: ExprRegion(
                            Id {
                                value: 59,
                            },
                        ),
                        body: Ok(
                            19,
                        ),
                    },
                ),
            ),
        ],
    },
)