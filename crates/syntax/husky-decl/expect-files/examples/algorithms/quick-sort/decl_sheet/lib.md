Ok(
    DeclSheet {
        decls: [
            (
                FormPath(`quick_sort::quick_sort`, `Function`),
                Err(
                    Expr(
                        ExpectRightParenthesis(
                            TokenIdx(
                                9,
                            ),
                        ),
                    ),
                ),
            ),
            (
                FormPath(`quick_sort::quick_sort_aux`, `Function`),
                Err(
                    Expr(
                        ExpectRightParenthesis(
                            TokenIdx(
                                48,
                            ),
                        ),
                    ),
                ),
            ),
            (
                FormPath(`quick_sort::partition`, `Function`),
                Err(
                    Expr(
                        ExpectRightParenthesis(
                            TokenIdx(
                                109,
                            ),
                        ),
                    ),
                ),
            ),
            (
                FormPath(`quick_sort::quick_sort_works_for_integers`, `Feature`),
                Ok(
                    Form(
                        Feature(
                            FeatureDecl {
                                path: FormPath(`quick_sort::quick_sort_works_for_integers`, `Feature`),
                            },
                        ),
                    ),
                ),
            ),
            (
                FormPath(`quick_sort::quick_sort_works_for_strs`, `Feature`),
                Ok(
                    Form(
                        Feature(
                            FeatureDecl {
                                path: FormPath(`quick_sort::quick_sort_works_for_strs`, `Feature`),
                            },
                        ),
                    ),
                ),
            ),
        ],
    },
)