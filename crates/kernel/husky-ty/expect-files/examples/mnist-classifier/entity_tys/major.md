[
    (
        FormPath(`mnist_classifier::major::connected_components`, `Feature`),
        Ok(
            Application(
                TermApplication {
                    function: Entity(
                        TypePath(`core::vec::Vec`, `Alien`),
                    ),
                    argument: Entity(
                        TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                    ),
                },
            ),
        ),
    ),
    (
        FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
        Ok(
            Entity(
                TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
            ),
        ),
    ),
    (
        FormPath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Feature`),
        Ok(
            Entity(
                TypePath(`core::num::f32`, `Alien`),
            ),
        ),
    ),
    (
        FormPath(`mnist_classifier::major::major_raw_contours`, `Feature`),
        Ok(
            Application(
                TermApplication {
                    function: Entity(
                        TypePath(`core::vec::Vec`, `Alien`),
                    ),
                    argument: Entity(
                        TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                    ),
                },
            ),
        ),
    ),
    (
        FormPath(`mnist_classifier::major::major_raw_contour`, `Feature`),
        Ok(
            Entity(
                TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
            ),
        ),
    ),
    (
        FormPath(`mnist_classifier::major::major_line_segment_sketch`, `Feature`),
        Ok(
            Entity(
                TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
            ),
        ),
    ),
    (
        FormPath(`mnist_classifier::major::major_concave_components`, `Feature`),
        Ok(
            Application(
                TermApplication {
                    function: Entity(
                        TypePath(`core::vec::Vec`, `Alien`),
                    ),
                    argument: Entity(
                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                    ),
                },
            ),
        ),
    ),
]