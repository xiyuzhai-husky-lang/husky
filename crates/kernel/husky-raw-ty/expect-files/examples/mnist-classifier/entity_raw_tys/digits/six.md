[
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Form(
                FormPath(`mnist_classifier::digits::six::six_match`, `Feature`),
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
                FormPath(`mnist_classifier::digits::six::six_match_refined1`, `Feature`),
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
                FormPath(`mnist_classifier::digits::six::upmost`, `Fn`),
            ),
        ),
        Ok(
            RawTerm(`Fp(~ mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent) -> core::option::Option core::num::f32`),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Form(
                FormPath(`mnist_classifier::digits::six::bottom1`, `Fn`),
            ),
        ),
        Ok(
            RawTerm(`Fp(~ mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent) -> core::option::Option core::num::f32`),
        ),
    ),
]