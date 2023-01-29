[
    ExprTypeRegion {
        path: Decl(
            Entity(
                ModuleItem(
                    Type(
                        TypePath(
                            Id {
                                value: 43,
                            },
                        ),
                    ),
                ),
            ),
        ),
        expr_ty_infos: [],
    },
    ExprTypeRegion {
        path: Decl(
            Entity(
                ModuleItem(
                    Type(
                        TypePath(
                            Id {
                                value: 44,
                            },
                        ),
                    ),
                ),
            ),
        ),
        expr_ty_infos: [],
    },
    ExprTypeRegion {
        path: Decl(
            Entity(
                ModuleItem(
                    Type(
                        TypePath(
                            Id {
                                value: 45,
                            },
                        ),
                    ),
                ),
            ),
        ),
        expr_ty_infos: [],
    },
    ExprTypeRegion {
        path: Decl(
            ImplBlock(
                ImplBlock(
                    Id {
                        value: 30,
                    },
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
        ],
    },
    ExprTypeRegion {
        path: Decl(
            AssociatedItem(
                AssociatedItem(
                    Id {
                        value: 79,
                    },
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
                        BinaryOpnFirstArgumentTypeNotInferred,
                    ),
                ),
                opt_expectation: None,
            },
            ExprTypeInfo {
                ty_result: Err(
                    Derived(
                        BinaryOpnFirstArgumentTypeNotInferred,
                    ),
                ),
                opt_expectation: None,
            },
        ],
    },
]