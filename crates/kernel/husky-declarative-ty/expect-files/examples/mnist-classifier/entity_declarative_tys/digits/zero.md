[
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Form(
                FugitivePath(`mnist_classifier::digits::zero::open_one_match`, `Val`),
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
                FugitivePath(`mnist_classifier::digits::zero::almost_closed`, `Fn`),
            ),
        ),
        Ok(
            DeclarativeTerm(`Fp(~ mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent) -> core::option::Option core::num::f32`),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Form(
                FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
            ),
        ),
        Err(
            DeclarativeTypeError::Derived(
                DerivedDeclarativeTypeError::SignatureError,
            ),
        ),
    ),
]