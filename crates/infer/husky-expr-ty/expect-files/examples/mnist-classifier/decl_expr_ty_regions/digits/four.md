[
    ExprTypeRegion {
        path: RegionPath::Decl(
            DeclExprPath::Entity(
                FormPath(`mnist_classifier::digits::four::left_components`, `Feature`),
            ),
        ),
        expr_ty_infos: [
            ExprTypeInfo {
                ty_result: Err(
                    Derived(
                        ExprError,
                    ),
                ),
                opt_expectation: None,
            },
        ],
    },
    ExprTypeRegion {
        path: RegionPath::Decl(
            DeclExprPath::Entity(
                FormPath(`mnist_classifier::digits::four::left_coordinate_max`, `Function`),
            ),
        ),
        expr_ty_infos: [
            ExprTypeInfo {
                ty_result: Ok(
                    Resolved(
                        Application(
                            TermApplication(
                                Id {
                                    value: 1,
                                },
                            ),
                        ),
                    ),
                ),
                opt_expectation: None,
            },
            ExprTypeInfo {
                ty_result: Ok(
                    Resolved(
                        Application(
                            TermApplication(
                                Id {
                                    value: 1,
                                },
                            ),
                        ),
                    ),
                ),
                opt_expectation: None,
            },
        ],
    },
    ExprTypeRegion {
        path: RegionPath::Decl(
            DeclExprPath::Entity(
                FormPath(`mnist_classifier::digits::four::components_max_downwards`, `Feature`),
            ),
        ),
        expr_ty_infos: [
            ExprTypeInfo {
                ty_result: Err(
                    Derived(
                        ExprError,
                    ),
                ),
                opt_expectation: None,
            },
        ],
    },
    ExprTypeRegion {
        path: RegionPath::Decl(
            DeclExprPath::Entity(
                FormPath(`mnist_classifier::digits::four::components_max_heights`, `Feature`),
            ),
        ),
        expr_ty_infos: [
            ExprTypeInfo {
                ty_result: Err(
                    Derived(
                        ExprError,
                    ),
                ),
                opt_expectation: None,
            },
        ],
    },
    ExprTypeRegion {
        path: RegionPath::Decl(
            DeclExprPath::Entity(
                FormPath(`mnist_classifier::digits::four::is_four`, `Feature`),
            ),
        ),
        expr_ty_infos: [
            ExprTypeInfo {
                ty_result: Err(
                    Derived(
                        ExprError,
                    ),
                ),
                opt_expectation: None,
            },
            ExprTypeInfo {
                ty_result: Err(
                    Derived(
                        PrefixOperandTypeNotInferred,
                    ),
                ),
                opt_expectation: None,
            },
        ],
    },
    ExprTypeRegion {
        path: RegionPath::Decl(
            DeclExprPath::Entity(
                FormPath(`mnist_classifier::digits::four::displacement_downwards`, `Function`),
            ),
        ),
        expr_ty_infos: [
            ExprTypeInfo {
                ty_result: Ok(
                    Resolved(
                        Application(
                            TermApplication(
                                Id {
                                    value: 1,
                                },
                            ),
                        ),
                    ),
                ),
                opt_expectation: None,
            },
            ExprTypeInfo {
                ty_result: Ok(
                    Resolved(
                        Application(
                            TermApplication(
                                Id {
                                    value: 1,
                                },
                            ),
                        ),
                    ),
                ),
                opt_expectation: None,
            },
        ],
    },
    ExprTypeRegion {
        path: RegionPath::Decl(
            DeclExprPath::Entity(
                FormPath(`mnist_classifier::digits::four::cc_box_heights`, `Function`),
            ),
        ),
        expr_ty_infos: [
            ExprTypeInfo {
                ty_result: Ok(
                    Resolved(
                        Application(
                            TermApplication(
                                Id {
                                    value: 1,
                                },
                            ),
                        ),
                    ),
                ),
                opt_expectation: None,
            },
            ExprTypeInfo {
                ty_result: Ok(
                    Resolved(
                        Application(
                            TermApplication(
                                Id {
                                    value: 1,
                                },
                            ),
                        ),
                    ),
                ),
                opt_expectation: None,
            },
        ],
    },
]