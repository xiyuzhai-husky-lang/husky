```rust
[
    (
        ItemPath::MajorItem(
            MajorItemPath::Form(
                FormPath(`mnist_classifier::major::connected_components`, `Val`),
            ),
        ),
        Ok(
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Form(
                    FormEthTemplate::Val(
                        MajorValEthTemplate {
                            path: FormPath(`mnist_classifier::major::connected_components`, `Val`),
                            return_ty: EthTerm(`Vec ConnectedComponent`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::MajorItem(
            MajorItemPath::Form(
                FormPath(`mnist_classifier::major::major_connected_component`, `Val`),
            ),
        ),
        Ok(
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Form(
                    FormEthTemplate::Val(
                        MajorValEthTemplate {
                            path: FormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                            return_ty: EthTerm(`Leash ConnectedComponent`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::MajorItem(
            MajorItemPath::Form(
                FormPath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Val`),
            ),
        ),
        Ok(
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Form(
                    FormEthTemplate::Val(
                        MajorValEthTemplate {
                            path: FormPath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Val`),
                            return_ty: EthTerm(`f32`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::MajorItem(
            MajorItemPath::Form(
                FormPath(`mnist_classifier::major::major_raw_contours`, `Val`),
            ),
        ),
        Ok(
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Form(
                    FormEthTemplate::Val(
                        MajorValEthTemplate {
                            path: FormPath(`mnist_classifier::major::major_raw_contours`, `Val`),
                            return_ty: EthTerm(`Leash Vec RawContour`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::MajorItem(
            MajorItemPath::Form(
                FormPath(`mnist_classifier::major::major_raw_contour`, `Val`),
            ),
        ),
        Ok(
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Form(
                    FormEthTemplate::Val(
                        MajorValEthTemplate {
                            path: FormPath(`mnist_classifier::major::major_raw_contour`, `Val`),
                            return_ty: EthTerm(`Leash RawContour`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::MajorItem(
            MajorItemPath::Form(
                FormPath(`mnist_classifier::major::major_line_segment_sketch`, `Val`),
            ),
        ),
        Ok(
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Form(
                    FormEthTemplate::Val(
                        MajorValEthTemplate {
                            path: FormPath(`mnist_classifier::major::major_line_segment_sketch`, `Val`),
                            return_ty: EthTerm(`Leash LineSegmentSketch`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::MajorItem(
            MajorItemPath::Form(
                FormPath(`mnist_classifier::major::major_concave_components`, `Val`),
            ),
        ),
        Ok(
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Form(
                    FormEthTemplate::Val(
                        MajorValEthTemplate {
                            path: FormPath(`mnist_classifier::major::major_concave_components`, `Val`),
                            return_ty: EthTerm(`Leash Vec ConcaveComponent`),
                        },
                    ),
                ),
            ),
        ),
    ),
]
```