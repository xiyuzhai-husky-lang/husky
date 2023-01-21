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
                                value: 47,
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
                                value: 48,
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
                                value: 49,
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
                                value: 50,
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
                                value: 51,
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