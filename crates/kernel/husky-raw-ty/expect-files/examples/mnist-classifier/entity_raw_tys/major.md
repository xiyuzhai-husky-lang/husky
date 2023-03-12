[
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Form(
                FormPath(`mnist_classifier::major::connected_components`, `Feature`),
            ),
        ),
        Ok(
            RawTerm(`core::list::List mnist_classifier::connected_component::ConnectedComponent`),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Form(
                FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
            ),
        ),
        Ok(
            RawTerm(`mnist_classifier::connected_component::ConnectedComponent`),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Form(
                FormPath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Feature`),
            ),
        ),
        Ok(
            RawTerm(`core::num::f32`),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Form(
                FormPath(`mnist_classifier::major::major_raw_contours`, `Feature`),
            ),
        ),
        Ok(
            RawTerm(`core::list::List mnist_classifier::raw_contour::RawContour`),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Form(
                FormPath(`mnist_classifier::major::major_raw_contour`, `Feature`),
            ),
        ),
        Ok(
            RawTerm(`mnist_classifier::raw_contour::RawContour`),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Form(
                FormPath(`mnist_classifier::major::major_line_segment_sketch`, `Feature`),
            ),
        ),
        Ok(
            RawTerm(`mnist_classifier::line_segment_sketch::LineSegmentSketch`),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Form(
                FormPath(`mnist_classifier::major::major_concave_components`, `Feature`),
            ),
        ),
        Ok(
            RawTerm(`core::list::List mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`),
        ),
    ),
]