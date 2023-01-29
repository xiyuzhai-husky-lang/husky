[
    ExprTypeRegion {
        path: Decl(
            Entity(
                ModuleItem(
                    Form(
                        FormPath(
                            Id {
                                value: 25,
                            },
                        ),
                    ),
                ),
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
        path: Decl(
            Entity(
                ModuleItem(
                    Form(
                        FormPath(
                            Id {
                                value: 26,
                            },
                        ),
                    ),
                ),
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
        path: Decl(
            Entity(
                ModuleItem(
                    Form(
                        FormPath(
                            Id {
                                value: 27,
                            },
                        ),
                    ),
                ),
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
]