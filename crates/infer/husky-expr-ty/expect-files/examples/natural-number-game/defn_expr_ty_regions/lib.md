[
    ExprTypeRegion {
        path: RegionPath::Defn(
            DefnExprPath::AssociatedItem(
                AssociatedItem {
                    id: AssociatedItemId {
                        impl_block_id: ImplBlockId {
                            module_path: `natural_number_game`,
                            impl_block_kind: ImplBlockKind::Type {
                                ty: TypePath(`natural_number_game::Nat`, `Inductive`),
                            },
                        },
                        ident: `add`,
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
                            impl_block_kind: ImplBlockKind::Type {
                                ty: TypePath(`natural_number_game::Nat`, `Inductive`),
                            },
                        },
                        ast_idx: 6,
                        body: ArenaIdxRange(
                            0..3,
                        ),
                        variant: ImplBlockVariant::Type {
                            ty: TypePath(`natural_number_game::Nat`, `Inductive`),
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
        expr_ty_infos: [],
    },
]