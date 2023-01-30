[
    ExprTypeRegion {
        path: RegionPath::Decl(
            DeclExprPath::Entity(
                TypePath(`mnist_classifier::line_segment_sketch::convex_component::ConvexComponent`, `Struct`),
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
                        BoxListApplicationFirstArgumentError,
                    ),
                ),
                opt_expectation: None,
            },
        ],
    },
    ExprTypeRegion {
        path: RegionPath::Decl(
            DeclExprPath::ImplBlock(
                ImplBlockId {
                    module_path: `mnist_classifier::line_segment_sketch::convex_component`,
                    impl_block_kind: ImplBlockKind::Type {
                        ty: TypePath(`mnist_classifier::line_segment_sketch::convex_component::ConvexComponent`, `Struct`),
                    },
                },
            ),
        ),
        expr_ty_infos: [
            ExprTypeInfo {
                ty_result: Err(
                    Derived(
                        EntityTypeError,
                    ),
                ),
                opt_expectation: None,
            },
        ],
    },
]