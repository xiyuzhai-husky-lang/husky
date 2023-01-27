[
    ExprTypeRegion {
        path: Decl(
            Entity(
                TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
            ),
        ),
    },
    ExprTypeRegion {
        path: Decl(
            ImplBlock(
                ImplBlock {
                    id: ImplBlockId {
                        module_path: `mnist_classifier::line_segment_sketch::line_segment`,
                        impl_block_kind: Type {
                            ty: TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                        },
                    },
                    ast_idx: 17,
                    body: ArenaIdxRange(
                        13..15,
                    ),
                    variant: Type {
                        ty: TypePath(
                            Id {
                                value: 33,
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
                                    value: 40,
                                },
                            ),
                            impl_block_kind: Type {
                                ty: TypePath(
                                    Id {
                                        value: 33,
                                    },
                                ),
                            },
                        },
                        ident: Identifier(
                            Word(
                                Id {
                                    value: 196,
                                },
                            ),
                        ),
                    },
                    path: Some(
                        TypeItemPath {
                            ty: TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                            ident: `displacement`,
                            ty_item_kind: Method,
                        },
                    ),
                    impl_block: ImplBlock {
                        id: ImplBlockId {
                            module_path: `mnist_classifier::line_segment_sketch::line_segment`,
                            impl_block_kind: Type {
                                ty: TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                            },
                        },
                        ast_idx: 17,
                        body: ArenaIdxRange(
                            13..15,
                        ),
                        variant: Type {
                            ty: TypePath(
                                Id {
                                    value: 33,
                                },
                            ),
                        },
                    },
                    ast_idx: 13,
                    ident: `displacement`,
                    associated_item_kind: TypeItem(
                        Method,
                    ),
                    accessibility: PubicUnder(
                        `mnist_classifier::line_segment_sketch::line_segment`,
                    ),
                    is_generic: false,
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
                                    value: 40,
                                },
                            ),
                            impl_block_kind: Type {
                                ty: TypePath(
                                    Id {
                                        value: 33,
                                    },
                                ),
                            },
                        },
                        ident: Identifier(
                            Word(
                                Id {
                                    value: 328,
                                },
                            ),
                        ),
                    },
                    path: Some(
                        TypeItemPath {
                            ty: TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                            ident: `dist_to_point`,
                            ty_item_kind: Method,
                        },
                    ),
                    impl_block: ImplBlock {
                        id: ImplBlockId {
                            module_path: `mnist_classifier::line_segment_sketch::line_segment`,
                            impl_block_kind: Type {
                                ty: TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                            },
                        },
                        ast_idx: 17,
                        body: ArenaIdxRange(
                            13..15,
                        ),
                        variant: Type {
                            ty: TypePath(
                                Id {
                                    value: 33,
                                },
                            ),
                        },
                    },
                    ast_idx: 14,
                    ident: `dist_to_point`,
                    associated_item_kind: TypeItem(
                        Method,
                    ),
                    accessibility: PubicUnder(
                        `mnist_classifier::line_segment_sketch::line_segment`,
                    ),
                    is_generic: false,
                },
            ),
        ),
    },
]