Ok(
    DeclSheet {
        decls: [
            (
                FormPath(`quick_sort::quick_sort`, `Function`),
                Err(
                    Expr(
                        MissingRightAngleBracket {
                            langle_token_idx: TokenIdx(
                                3,
                            ),
                        },
                    ),
                ),
            ),
            (
                FormPath(`quick_sort::quick_sort_aux`, `Function`),
                Err(
                    Expr(
                        MissingRightAngleBracket {
                            langle_token_idx: TokenIdx(
                                41,
                            ),
                        },
                    ),
                ),
            ),
            (
                FormPath(`quick_sort::partition`, `Function`),
                Err(
                    Expr(
                        MissingRightAngleBracket {
                            langle_token_idx: TokenIdx(
                                101,
                            ),
                        },
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