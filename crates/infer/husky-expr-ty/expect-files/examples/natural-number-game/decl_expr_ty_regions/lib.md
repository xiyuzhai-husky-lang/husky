[
    ExprTypeRegion {
        path: Decl(
            Entity(
                TypePath(`natural_number_game::Nat`, `Inductive`),
            ),
        ),
    },
    ExprTypeRegion {
        path: Decl(
            Entity(
                TypePath(`natural_number_game::OddNat`, `Structure`),
            ),
        ),
    },
    ExprTypeRegion {
        path: Decl(
            Entity(
                TypePath(`natural_number_game::EvenNat`, `Structure`),
            ),
        ),
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
                                value: 35,
                            },
                        ),
                    },
                },
            ),
        ),
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
                                        value: 35,
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
                                    value: 35,
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
    },
]