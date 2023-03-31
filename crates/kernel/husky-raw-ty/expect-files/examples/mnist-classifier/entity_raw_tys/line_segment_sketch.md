[
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Type(
                TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
            ),
        ),
        Err(
            RawTypeError::Derived(
                DerivedRawTypeError::TypeOntologyDeclError {
                    path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                },
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
                FormPath(`mnist_classifier::line_segment_sketch::go_right`, `Fn`),
            ),
        ),
        Ok(
            RawTerm(`Fp(mnist_classifier::geom2d::Vector2d, core::num::f32) -> mnist_classifier::geom2d::Vector2d`),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Form(
                FormPath(`mnist_classifier::line_segment_sketch::go_left`, `Fn`),
            ),
        ),
        Ok(
            RawTerm(`Fp(mnist_classifier::geom2d::Vector2d, core::num::f32) -> mnist_classifier::geom2d::Vector2d`),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Form(
                FormPath(`mnist_classifier::line_segment_sketch::extend_end`, `Fn`),
            ),
        ),
        Ok(
            RawTerm(`Fp(~ mnist_classifier::raw_contour::RawContour, core::num::i32, core::num::f32) -> core::num::i32`),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Form(
                FormPath(`mnist_classifier::line_segment_sketch::extend_start`, `Fn`),
            ),
        ),
        Ok(
            RawTerm(`Fp(~ mnist_classifier::raw_contour::RawContour, core::num::i32, core::num::i32, core::num::f32) -> core::num::i32`),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Form(
                FormPath(`mnist_classifier::line_segment_sketch::find_line_segments`, `Fn`),
            ),
        ),
        Ok(
            RawTerm(`Fp(~ mnist_classifier::raw_contour::RawContour, core::num::f32) -> [] mnist_classifier::line_segment_sketch::LineSegmentStroke`),
        ),
    ),
]