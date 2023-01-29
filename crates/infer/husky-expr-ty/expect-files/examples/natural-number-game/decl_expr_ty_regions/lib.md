[
    ExprTypeRegion {
        path: Decl(
            Entity(
                TypePath(`natural_number_game::Nat`, `Inductive`),
            ),
        ),
        expr_ty_infos: [],
    },
    ExprTypeRegion {
        path: Decl(
            Entity(
                TypePath(`natural_number_game::OddNat`, `Structure`),
            ),
        ),
        expr_ty_infos: [],
    },
    ExprTypeRegion {
        path: Decl(
            Entity(
                TypePath(`natural_number_game::EvenNat`, `Structure`),
            ),
        ),
        expr_ty_infos: [],
    },
    ExprTypeRegion {
        path: Decl(
            ImplBlock(
                ImplBlock {
                    id: ImplBlockId {
                        module_path: `natural_number_game`,
                        impl_block_kind: Type {
                            ty: TypePath(`natural_number_game::Nat`, `Inductive`),
                        },
                    },
                    ast_idx: 6,
                    body: ArenaIdxRange(
                        0..3,
                    ),
                    variant: Type {
                        ty: TypePath(
                            Id {
                                value: 43,
                            },
                        ),
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
        path: Decl(
            AssociatedItem(
                AssociatedItem {
                    id: AssociatedItemId {
                        impl_block_id: ImplBlockId {
                            module_path: ModulePath(
                                Id {
                                    value: 43,
                                },
                            ),
                            impl_block_kind: Type {
                                ty: TypePath(
                                    Id {
                                        value: 43,
                                    },
                                ),
                            },
                        },
                        ident: Identifier(
                            Word(
                                Id {
                                    value: 35,
                                },
                            ),
                        ),
                    },
                    path: Some(
                        TypeItemPath {
                            ty: TypePath(`natural_number_game::Nat`, `Inductive`),
                            ident: `add`,
                            ty_item_kind: Memo,
                        },
                    ),
                    impl_block: ImplBlock {
                        id: ImplBlockId {
                            module_path: `natural_number_game`,
                            impl_block_kind: Type {
                                ty: TypePath(`natural_number_game::Nat`, `Inductive`),
                            },
                        },
                        ast_idx: 6,
                        body: ArenaIdxRange(
                            0..3,
                        ),
                        variant: Type {
                            ty: TypePath(
                                Id {
                                    value: 43,
                                },
                            ),
                        },
                    },
                    ast_idx: 0,
                    ident: `add`,
                    associated_item_kind: TypeItem(
                        Memo,
                    ),
                    accessibility: PubicUnder(
                        `natural_number_game`,
                    ),
                    is_generic: false,
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