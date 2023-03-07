[
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Form(
                FormPath(`mnist_classifier::digits::nine::nine_match`, `Feature`),
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
                FormPath(`mnist_classifier::digits::nine::nine_match_refine`, `Feature`),
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
                FormPath(`mnist_classifier::digits::nine::is_nine`, `Feature`),
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
                FormPath(`mnist_classifier::digits::nine::downmost`, `Function`),
            ),
        ),
        Ok(
            RawTerm(`Fp(~ mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent) -> core::option::Option core::num::f32`),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Form(
                FormPath(`mnist_classifier::digits::nine::big_cc`, `Function`),
            ),
        ),
        Ok(
            RawTerm(`Fp(~ mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent) -> core::option::Option core::num::f32`),
        ),
    ),
]