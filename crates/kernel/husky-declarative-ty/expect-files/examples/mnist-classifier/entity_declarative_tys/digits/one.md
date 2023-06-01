[
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Form(
                FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
            ),
        ),
        Err(
            DeclarativeTypeError::Derived(
                DerivedDeclarativeTypeError::SignatureError,
            ),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Form(
                FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
            ),
        ),
        Err(
            DeclarativeTypeError::Derived(
                DerivedDeclarativeTypeError::SignatureError,
            ),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Form(
                FugitivePath(`mnist_classifier::digits::one::upmost`, `Fn`),
            ),
        ),
        Ok(
            DeclarativeTerm(`Fp(~ mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent) -> core::option::Option core::num::f32`),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Form(
                FugitivePath(`mnist_classifier::digits::one::downmost`, `Fn`),
            ),
        ),
        Ok(
            DeclarativeTerm(`Fp(~ mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent) -> core::option::Option core::num::f32`),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Form(
                FugitivePath(`mnist_classifier::digits::one::hat`, `Fn`),
            ),
        ),
        Ok(
            DeclarativeTerm(`Fp(~ mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent) -> core::option::Option core::num::f32`),
        ),
    ),
]