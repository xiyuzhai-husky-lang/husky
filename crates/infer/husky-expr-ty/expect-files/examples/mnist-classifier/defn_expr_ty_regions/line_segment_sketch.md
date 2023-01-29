[
    ExprTypeRegion {
        path: Defn(
            Entity(
                FormPath(`mnist_classifier::line_segment_sketch::go_right`, `Function`),
            ),
        ),
        expr_ty_infos: [],
    },
    ExprTypeRegion {
        path: Defn(
            Entity(
                FormPath(`mnist_classifier::line_segment_sketch::go_left`, `Function`),
            ),
        ),
        expr_ty_infos: [],
    },
    ExprTypeRegion {
        path: Defn(
            Entity(
                FormPath(`mnist_classifier::line_segment_sketch::extend_end`, `Function`),
            ),
        ),
        expr_ty_infos: [],
    },
    ExprTypeRegion {
        path: Defn(
            Entity(
                FormPath(`mnist_classifier::line_segment_sketch::extend_start`, `Function`),
            ),
        ),
        expr_ty_infos: [],
    },
    ExprTypeRegion {
        path: Defn(
            Entity(
                FormPath(`mnist_classifier::line_segment_sketch::find_line_segments`, `Function`),
            ),
        ),
        expr_ty_infos: [],
    },
    ExprTypeRegion {
        path: Defn(
            AssociatedItem(
                AssociatedItem {
                    id: AssociatedItemId {
                        impl_block_id: ImplBlockId {
                            module_path: ModulePath(
                                Id {
                                    value: 36,
                                },
                            ),
                            impl_block_kind: Type {
                                ty: TypePath(
                                    Id {
                                        value: 37,
                                    },
                                ),
                            },
                        },
                        ident: Identifier(
                            Word(
                                Id {
                                    value: 198,
                                },
                            ),
                        ),
                    },
                    path: Some(
                        TypeItemPath {
                            ty: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                            ident: `new`,
                            ty_item_kind: Method,
                        },
                    ),
                    impl_block: ImplBlock {
                        id: ImplBlockId {
                            module_path: `mnist_classifier::line_segment_sketch`,
                            impl_block_kind: Type {
                                ty: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                            },
                        },
                        ast_idx: 170,
                        body: ArenaIdxRange(
                            3..7,
                        ),
                        variant: Type {
                            ty: TypePath(
                                Id {
                                    value: 37,
                                },
                            ),
                        },
                    },
                    ast_idx: 3,
                    ident: `new`,
                    associated_item_kind: TypeItem(
                        Method,
                    ),
                    accessibility: Public,
                    is_generic: false,
                },
            ),
        ),
        expr_ty_infos: [],
    },
    ExprTypeRegion {
        path: Defn(
            AssociatedItem(
                AssociatedItem {
                    id: AssociatedItemId {
                        impl_block_id: ImplBlockId {
                            module_path: ModulePath(
                                Id {
                                    value: 36,
                                },
                            ),
                            impl_block_kind: Type {
                                ty: TypePath(
                                    Id {
                                        value: 37,
                                    },
                                ),
                            },
                        },
                        ident: Identifier(
                            Word(
                                Id {
                                    value: 212,
                                },
                            ),
                        ),
                    },
                    path: Some(
                        TypeItemPath {
                            ty: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                            ident: `displacement`,
                            ty_item_kind: Method,
                        },
                    ),
                    impl_block: ImplBlock {
                        id: ImplBlockId {
                            module_path: `mnist_classifier::line_segment_sketch`,
                            impl_block_kind: Type {
                                ty: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                            },
                        },
                        ast_idx: 170,
                        body: ArenaIdxRange(
                            3..7,
                        ),
                        variant: Type {
                            ty: TypePath(
                                Id {
                                    value: 37,
                                },
                            ),
                        },
                    },
                    ast_idx: 4,
                    ident: `displacement`,
                    associated_item_kind: TypeItem(
                        Method,
                    ),
                    accessibility: PubicUnder(
                        `mnist_classifier::line_segment_sketch`,
                    ),
                    is_generic: false,
                },
            ),
        ),
        expr_ty_infos: [],
    },
    ExprTypeRegion {
        path: Defn(
            AssociatedItem(
                AssociatedItem {
                    id: AssociatedItemId {
                        impl_block_id: ImplBlockId {
                            module_path: ModulePath(
                                Id {
                                    value: 36,
                                },
                            ),
                            impl_block_kind: Type {
                                ty: TypePath(
                                    Id {
                                        value: 38,
                                    },
                                ),
                            },
                        },
                        ident: Identifier(
                            Word(
                                Id {
                                    value: 302,
                                },
                            ),
                        ),
                    },
                    path: Some(
                        TypeItemPath {
                            ty: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                            ident: `concave_components`,
                            ty_item_kind: Memo,
                        },
                    ),
                    impl_block: ImplBlock {
                        id: ImplBlockId {
                            module_path: `mnist_classifier::line_segment_sketch`,
                            impl_block_kind: Type {
                                ty: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                            },
                        },
                        ast_idx: 172,
                        body: ArenaIdxRange(
                            21..26,
                        ),
                        variant: Type {
                            ty: TypePath(
                                Id {
                                    value: 38,
                                },
                            ),
                        },
                    },
                    ast_idx: 21,
                    ident: `concave_components`,
                    associated_item_kind: TypeItem(
                        Memo,
                    ),
                    accessibility: PubicUnder(
                        `mnist_classifier::line_segment_sketch`,
                    ),
                    is_generic: false,
                },
            ),
        ),
        expr_ty_infos: [],
    },
    ExprTypeRegion {
        path: Defn(
            AssociatedItem(
                AssociatedItem {
                    id: AssociatedItemId {
                        impl_block_id: ImplBlockId {
                            module_path: ModulePath(
                                Id {
                                    value: 36,
                                },
                            ),
                            impl_block_kind: Type {
                                ty: TypePath(
                                    Id {
                                        value: 38,
                                    },
                                ),
                            },
                        },
                        ident: Identifier(
                            Word(
                                Id {
                                    value: 199,
                                },
                            ),
                        ),
                    },
                    path: Some(
                        TypeItemPath {
                            ty: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                            ident: `bounding_box`,
                            ty_item_kind: Memo,
                        },
                    ),
                    impl_block: ImplBlock {
                        id: ImplBlockId {
                            module_path: `mnist_classifier::line_segment_sketch`,
                            impl_block_kind: Type {
                                ty: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                            },
                        },
                        ast_idx: 172,
                        body: ArenaIdxRange(
                            21..26,
                        ),
                        variant: Type {
                            ty: TypePath(
                                Id {
                                    value: 38,
                                },
                            ),
                        },
                    },
                    ast_idx: 22,
                    ident: `bounding_box`,
                    associated_item_kind: TypeItem(
                        Memo,
                    ),
                    accessibility: PubicUnder(
                        `mnist_classifier::line_segment_sketch`,
                    ),
                    is_generic: false,
                },
            ),
        ),
        expr_ty_infos: [],
    },
    ExprTypeRegion {
        path: Defn(
            AssociatedItem(
                AssociatedItem {
                    id: AssociatedItemId {
                        impl_block_id: ImplBlockId {
                            module_path: ModulePath(
                                Id {
                                    value: 36,
                                },
                            ),
                            impl_block_kind: Type {
                                ty: TypePath(
                                    Id {
                                        value: 38,
                                    },
                                ),
                            },
                        },
                        ident: Identifier(
                            Word(
                                Id {
                                    value: 198,
                                },
                            ),
                        ),
                    },
                    path: Some(
                        TypeItemPath {
                            ty: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                            ident: `new`,
                            ty_item_kind: Method,
                        },
                    ),
                    impl_block: ImplBlock {
                        id: ImplBlockId {
                            module_path: `mnist_classifier::line_segment_sketch`,
                            impl_block_kind: Type {
                                ty: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                            },
                        },
                        ast_idx: 172,
                        body: ArenaIdxRange(
                            21..26,
                        ),
                        variant: Type {
                            ty: TypePath(
                                Id {
                                    value: 38,
                                },
                            ),
                        },
                    },
                    ast_idx: 23,
                    ident: `new`,
                    associated_item_kind: TypeItem(
                        Method,
                    ),
                    accessibility: Public,
                    is_generic: false,
                },
            ),
        ),
        expr_ty_infos: [],
    },
]