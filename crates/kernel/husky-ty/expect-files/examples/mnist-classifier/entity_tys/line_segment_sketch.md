[
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Type(
                TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
            ),
        ),
        Err(
            Derived(
                SignatureError,
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
            Term(`Type`),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Form(
                FormPath(`mnist_classifier::line_segment_sketch::go_right`, `Function`),
            ),
        ),
        Ok(
            Term(`Fp(Vector2d, f32) -> Vector2d`),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Form(
                FormPath(`mnist_classifier::line_segment_sketch::go_left`, `Function`),
            ),
        ),
        Ok(
            Term(`Fp(Vector2d, f32) -> Vector2d`),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Form(
                FormPath(`mnist_classifier::line_segment_sketch::extend_end`, `Function`),
            ),
        ),
        Ok(
            Term(`Fp(Ref 'eval RawContour, i32, f32) -> i32`),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Form(
                FormPath(`mnist_classifier::line_segment_sketch::extend_start`, `Function`),
            ),
        ),
        Ok(
            Term(`Fp(Ref 'eval RawContour, i32, i32, f32) -> i32`),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Form(
                FormPath(`mnist_classifier::line_segment_sketch::find_line_segments`, `Function`),
            ),
        ),
        Ok(
            Term(`Fp(Ref 'eval RawContour, f32) -> List LineSegmentStroke`),
        ),
    ),
]