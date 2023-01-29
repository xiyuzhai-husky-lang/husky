[
    ExprTypeRegion {
        path: Decl(
            Entity(
                TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
            ),
        ),
    },
    ExprTypeRegion {
        path: Decl(
            Entity(
                TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
            ),
        ),
    },
    ExprTypeRegion {
        path: Decl(
            Entity(
                FormPath(`mnist_classifier::raw_contour::get_pixel_pair`, `Function`),
            ),
        ),
    },
    ExprTypeRegion {
        path: Decl(
            Entity(
                FormPath(`mnist_classifier::raw_contour::get_pixel_to_the_left`, `Function`),
            ),
        ),
    },
    ExprTypeRegion {
        path: Decl(
            Entity(
                FormPath(`mnist_classifier::raw_contour::get_pixel_to_the_right`, `Function`),
            ),
        ),
    },
    ExprTypeRegion {
        path: Decl(
            Entity(
                FormPath(`mnist_classifier::raw_contour::get_inward_direction`, `Function`),
            ),
        ),
    },
    ExprTypeRegion {
        path: Decl(
            Entity(
                FormPath(`mnist_classifier::raw_contour::get_angle_change`, `Function`),
            ),
        ),
    },
    ExprTypeRegion {
        path: Decl(
            Entity(
                FormPath(`mnist_classifier::raw_contour::get_outward_direction`, `Function`),
            ),
        ),
    },
    ExprTypeRegion {
        path: Decl(
            Entity(
                TypePath(`mnist_classifier::raw_contour::StreakCache`, `Struct`),
            ),
        ),
    },
    ExprTypeRegion {
        path: Decl(
            Entity(
                FormPath(`mnist_classifier::raw_contour::get_concave_middle_point`, `Function`),
            ),
        ),
    },
    ExprTypeRegion {
        path: Decl(
            Entity(
                FormPath(`mnist_classifier::raw_contour::find_raw_contours`, `Function`),
            ),
        ),
    },
    ExprTypeRegion {
        path: Decl(
            ImplBlock(
                ImplBlock {
                    id: ImplBlockId {
                        module_path: `mnist_classifier::raw_contour`,
                        impl_block_kind: Type {
                            ty: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                        },
                    },
                    ast_idx: 200,
                    body: ArenaIdxRange(
                        27..34,
                    ),
                    variant: Type {
                        ty: TypePath(
                            Id {
                                value: 28,
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
                                    value: 42,
                                },
                            ),
                            impl_block_kind: Type {
                                ty: TypePath(
                                    Id {
                                        value: 28,
                                    },
                                ),
                            },
                        },
                        ident: Identifier(
                            Word(
                                Id {
                                    value: 112,
                                },
                            ),
                        ),
                    },
                    path: Some(
                        TypeItemPath {
                            ty: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                            ident: `line_segment_sketch`,
                            ty_item_kind: Memo,
                        },
                    ),
                    impl_block: ImplBlock {
                        id: ImplBlockId {
                            module_path: `mnist_classifier::raw_contour`,
                            impl_block_kind: Type {
                                ty: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                            },
                        },
                        ast_idx: 200,
                        body: ArenaIdxRange(
                            27..34,
                        ),
                        variant: Type {
                            ty: TypePath(
                                Id {
                                    value: 28,
                                },
                            ),
                        },
                    },
                    ast_idx: 27,
                    ident: `line_segment_sketch`,
                    associated_item_kind: TypeItem(
                        Memo,
                    ),
                    accessibility: PubicUnder(
                        `mnist_classifier::raw_contour`,
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
                                    value: 42,
                                },
                            ),
                            impl_block_kind: Type {
                                ty: TypePath(
                                    Id {
                                        value: 28,
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
                            ty: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                            ident: `bounding_box`,
                            ty_item_kind: Memo,
                        },
                    ),
                    impl_block: ImplBlock {
                        id: ImplBlockId {
                            module_path: `mnist_classifier::raw_contour`,
                            impl_block_kind: Type {
                                ty: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                            },
                        },
                        ast_idx: 200,
                        body: ArenaIdxRange(
                            27..34,
                        ),
                        variant: Type {
                            ty: TypePath(
                                Id {
                                    value: 28,
                                },
                            ),
                        },
                    },
                    ast_idx: 28,
                    ident: `bounding_box`,
                    associated_item_kind: TypeItem(
                        Memo,
                    ),
                    accessibility: PubicUnder(
                        `mnist_classifier::raw_contour`,
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
                                    value: 42,
                                },
                            ),
                            impl_block_kind: Type {
                                ty: TypePath(
                                    Id {
                                        value: 28,
                                    },
                                ),
                            },
                        },
                        ident: Identifier(
                            Word(
                                Id {
                                    value: 209,
                                },
                            ),
                        ),
                    },
                    path: Some(
                        TypeItemPath {
                            ty: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                            ident: `relative_bounding_box`,
                            ty_item_kind: Memo,
                        },
                    ),
                    impl_block: ImplBlock {
                        id: ImplBlockId {
                            module_path: `mnist_classifier::raw_contour`,
                            impl_block_kind: Type {
                                ty: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                            },
                        },
                        ast_idx: 200,
                        body: ArenaIdxRange(
                            27..34,
                        ),
                        variant: Type {
                            ty: TypePath(
                                Id {
                                    value: 28,
                                },
                            ),
                        },
                    },
                    ast_idx: 29,
                    ident: `relative_bounding_box`,
                    associated_item_kind: TypeItem(
                        Memo,
                    ),
                    accessibility: PubicUnder(
                        `mnist_classifier::raw_contour`,
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
                                    value: 42,
                                },
                            ),
                            impl_block_kind: Type {
                                ty: TypePath(
                                    Id {
                                        value: 28,
                                    },
                                ),
                            },
                        },
                        ident: Identifier(
                            Word(
                                Id {
                                    value: 151,
                                },
                            ),
                        ),
                    },
                    path: Some(
                        TypeItemPath {
                            ty: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                            ident: `contour_len`,
                            ty_item_kind: Memo,
                        },
                    ),
                    impl_block: ImplBlock {
                        id: ImplBlockId {
                            module_path: `mnist_classifier::raw_contour`,
                            impl_block_kind: Type {
                                ty: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                            },
                        },
                        ast_idx: 200,
                        body: ArenaIdxRange(
                            27..34,
                        ),
                        variant: Type {
                            ty: TypePath(
                                Id {
                                    value: 28,
                                },
                            ),
                        },
                    },
                    ast_idx: 30,
                    ident: `contour_len`,
                    associated_item_kind: TypeItem(
                        Memo,
                    ),
                    accessibility: PubicUnder(
                        `mnist_classifier::raw_contour`,
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
                                    value: 42,
                                },
                            ),
                            impl_block_kind: Type {
                                ty: TypePath(
                                    Id {
                                        value: 28,
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
                            ty: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                            ident: `displacement`,
                            ty_item_kind: Method,
                        },
                    ),
                    impl_block: ImplBlock {
                        id: ImplBlockId {
                            module_path: `mnist_classifier::raw_contour`,
                            impl_block_kind: Type {
                                ty: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                            },
                        },
                        ast_idx: 200,
                        body: ArenaIdxRange(
                            27..34,
                        ),
                        variant: Type {
                            ty: TypePath(
                                Id {
                                    value: 28,
                                },
                            ),
                        },
                    },
                    ast_idx: 31,
                    ident: `displacement`,
                    associated_item_kind: TypeItem(
                        Method,
                    ),
                    accessibility: PubicUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    is_generic: false,
                },
            ),
        ),
    },
]