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
                        expr_sheet: ExprSheet(
                            Id {
                                value: 62,
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
                        expr_sheet: ExprSheet(
                            Id {
                                value: 63,
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
                        expr_sheet: ExprSheet(
                            Id {
                                value: 64,
                            },
                        ),
                        body: Ok(
                            67,
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
                        expr_sheet: ExprSheet(
                            Id {
                                value: 65,
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
                        expr_sheet: ExprSheet(
                            Id {
                                value: 66,
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