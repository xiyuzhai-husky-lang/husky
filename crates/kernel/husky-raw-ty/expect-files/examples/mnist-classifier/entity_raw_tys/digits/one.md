[
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Form(
                FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Feature`),
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
                FormPath(`mnist_classifier::digits::one::is_one`, `Feature`),
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
                FormPath(`mnist_classifier::digits::one::upmost`, `Function`),
            ),
        ),
        Ok(
            RawTerm(`Fp(~ mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent) -> core::option::Option core::num::f32`),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Form(
                FormPath(`mnist_classifier::digits::one::downmost`, `Function`),
            ),
        ),
        Ok(
            RawTerm(`Fp(~ mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent) -> core::option::Option core::num::f32`),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Form(
                FormPath(`mnist_classifier::digits::one::hat`, `Function`),
            ),
        ),
        Ok(
            RawTerm(`Fp(~ mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent) -> core::option::Option core::num::f32`),
        ),
    ),
]