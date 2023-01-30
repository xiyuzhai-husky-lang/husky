[
    ExprTypeRegion {
        path: RegionPath::Decl(
            DeclExprPath::Entity(
                TypePath(`natural_number_game::Nat`, `Inductive`),
            ),
        ),
        expr_ty_infos: [],
    },
    ExprTypeRegion {
        path: RegionPath::Decl(
            DeclExprPath::Entity(
                TypePath(`natural_number_game::OddNat`, `Structure`),
            ),
        ),
        expr_ty_infos: [],
    },
    ExprTypeRegion {
        path: RegionPath::Decl(
            DeclExprPath::Entity(
                TypePath(`natural_number_game::EvenNat`, `Structure`),
            ),
        ),
        expr_ty_infos: [],
    },
    ExprTypeRegion {
        path: RegionPath::Decl(
            DeclExprPath::ImplBlock(
                ImplBlockId {
                    module_path: `natural_number_game`,
                    impl_block_kind: ImplBlockKind::Type {
                        ty: TypePath(`natural_number_game::Nat`, `Inductive`),
                    },
                },
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
        path: RegionPath::Decl(
            DeclExprPath::AssociatedItem(
                AssociatedItemId {
                    impl_block_id: ImplBlockId {
                        module_path: `natural_number_game`,
                        impl_block_kind: ImplBlockKind::Type {
                            ty: TypePath(`natural_number_game::Nat`, `Inductive`),
                        },
                    },
                    ident: `add`,
                },
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