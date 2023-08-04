Ok(
    [
        ItemPath::MajorItem(
            MajarItemPath::Type(
                TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
            ),
        ),
        ItemPath::ImplBlock(
            ImplBlockPath::TypeImplBlock(
                TypeImplBlockPath {
                    module_path: `mnist_classifier::line_segment_sketch::line_segment`,
                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                    disambiguator: 0,
                },
            ),
        ),
        ItemPath::AssociatedItem(
            AssociatedItemPath::TypeItem(
                TypeItemPath {
                    impl_block: TypeImplBlockPath {
                        module_path: `mnist_classifier::line_segment_sketch::line_segment`,
                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                        disambiguator: 0,
                    },
                    ident: `displacement`,
                    item_kind: MethodFn,
                },
            ),
        ),
        ItemPath::AssociatedItem(
            AssociatedItemPath::TypeItem(
                TypeItemPath {
                    impl_block: TypeImplBlockPath {
                        module_path: `mnist_classifier::line_segment_sketch::line_segment`,
                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                        disambiguator: 0,
                    },
                    ident: `dist_to_point`,
                    item_kind: MethodFn,
                },
            ),
        ),
    ],
)