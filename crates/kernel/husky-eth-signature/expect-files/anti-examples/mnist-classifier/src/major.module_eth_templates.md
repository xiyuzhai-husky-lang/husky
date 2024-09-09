```rust
[
    (
        ItemPath(`mnist_classifier::major::connected_components`),
        Ok(
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Form(
                    FormEthTemplate::Val(
                        MajorValEthTemplate {
                            path: MajorFormPath(`mnist_classifier::major::connected_components`, `Val`),
                            return_ty: EthTerm(`Vec ConnectedComponent`),
                            expr_ty: EthTerm(`Leash Vec ConnectedComponent`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::major::major_connected_component`),
        Ok(
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Form(
                    FormEthTemplate::Val(
                        MajorValEthTemplate {
                            path: MajorFormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                            return_ty: EthTerm(`Leash ConnectedComponent`),
                            expr_ty: EthTerm(`Leash ConnectedComponent`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`),
        Ok(
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Form(
                    FormEthTemplate::Val(
                        MajorValEthTemplate {
                            path: MajorFormPath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Val`),
                            return_ty: EthTerm(`f32`),
                            expr_ty: EthTerm(`f32`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::major::major_raw_contours`),
        Ok(
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Form(
                    FormEthTemplate::Val(
                        MajorValEthTemplate {
                            path: MajorFormPath(`mnist_classifier::major::major_raw_contours`, `Val`),
                            return_ty: EthTerm(`Leash Vec RawContour`),
                            expr_ty: EthTerm(`Leash Vec RawContour`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::major::major_raw_contour`),
        Ok(
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Form(
                    FormEthTemplate::Val(
                        MajorValEthTemplate {
                            path: MajorFormPath(`mnist_classifier::major::major_raw_contour`, `Val`),
                            return_ty: EthTerm(`Leash RawContour`),
                            expr_ty: EthTerm(`Leash RawContour`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::major::major_line_segment_sketch`),
        Ok(
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Form(
                    FormEthTemplate::Val(
                        MajorValEthTemplate {
                            path: MajorFormPath(`mnist_classifier::major::major_line_segment_sketch`, `Val`),
                            return_ty: EthTerm(`Leash LineSegmentSketch`),
                            expr_ty: EthTerm(`Leash LineSegmentSketch`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::major::major_concave_components`),
        Ok(
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Form(
                    FormEthTemplate::Val(
                        MajorValEthTemplate {
                            path: MajorFormPath(`mnist_classifier::major::major_concave_components`, `Val`),
                            return_ty: EthTerm(`Leash Vec ConcaveComponent`),
                            expr_ty: EthTerm(`Leash Vec ConcaveComponent`),
                        },
                    ),
                ),
            ),
        ),
    ),
]
```