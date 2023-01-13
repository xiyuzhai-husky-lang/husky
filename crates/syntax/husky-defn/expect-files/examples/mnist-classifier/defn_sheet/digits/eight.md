Ok(
    DefnSheet {
        defns: [
            Form(
                Feature(
                    FeatureDefn {
                        path: FormPath(`mnist_classifier::digits::eight::upper_mouth_match`, `Feature`),
                        decl: FeatureDecl(
                            Id {
                                value: 3,
                            },
                        ),
                        expr_sheet: ExprSheet(
                            Id {
                                value: 99,
                            },
                        ),
                        body: Ok(
                            5,
                        ),
                    },
                ),
            ),
            Form(
                Feature(
                    FeatureDefn {
                        path: FormPath(`mnist_classifier::digits::eight::is_eight`, `Feature`),
                        decl: FeatureDecl(
                            Id {
                                value: 4,
                            },
                        ),
                        expr_sheet: ExprSheet(
                            Id {
                                value: 100,
                            },
                        ),
                        body: Ok(
                            30,
                        ),
                    },
                ),
            ),
            Form(
                Function(
                    FunctionDefn {
                        path: FormPath(`mnist_classifier::digits::eight::big_mouth`, `Function`),
                        decl: FunctionDecl(
                            Id {
                                value: 7,
                            },
                        ),
                        expr_sheet: ExprSheet(
                            Id {
                                value: 101,
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