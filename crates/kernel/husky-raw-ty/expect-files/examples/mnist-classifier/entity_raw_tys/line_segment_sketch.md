[
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Type(
                TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
            ),
        ),
        Err(
            RawTypeError::Derived(
                DerivedRawTypeError::SignatureError,
            ),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Type(
                TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
            ),
        ),
        Ok(
            RawTerm(`Type`),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Form(
                FormPath(`mnist_classifier::line_segment_sketch::go_right`, `Function`),
            ),
        ),
        Ok(
            RawTerm(`Fp(mnist_classifier::geom2d::Vector2d, core::num::f32) -> mnist_classifier::geom2d::Vector2d`),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Form(
                FormPath(`mnist_classifier::line_segment_sketch::go_left`, `Function`),
            ),
        ),
        Ok(
            RawTerm(`Fp(mnist_classifier::geom2d::Vector2d, core::num::f32) -> mnist_classifier::geom2d::Vector2d`),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Form(
                FormPath(`mnist_classifier::line_segment_sketch::extend_end`, `Function`),
            ),
        ),
        Ok(
            RawTerm(`Fp(~ mnist_classifier::raw_contour::RawContour, core::num::i32, core::num::f32) -> core::num::i32`),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Form(
                FormPath(`mnist_classifier::line_segment_sketch::extend_start`, `Function`),
            ),
        ),
        Ok(
            RawTerm(`Fp(~ mnist_classifier::raw_contour::RawContour, core::num::i32, core::num::i32, core::num::f32) -> core::num::i32`),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Form(
                FormPath(`mnist_classifier::line_segment_sketch::find_line_segments`, `Function`),
            ),
        ),
        Ok(
            RawTerm(`Fp(~ mnist_classifier::raw_contour::RawContour, core::num::f32) -> core::list::List mnist_classifier::line_segment_sketch::LineSegmentStroke`),
        ),
    ),
]