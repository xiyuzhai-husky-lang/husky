[
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Type(
                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
            ModuleItemPath::Form(
                FormPath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Fn`),
            ),
        ),
        Ok(
            RawTerm(`Fp(core::mem::Ref mnist_classifier::line_segment_sketch::LineSegmentSketch) -> [] mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`),
        ),
    ),
]