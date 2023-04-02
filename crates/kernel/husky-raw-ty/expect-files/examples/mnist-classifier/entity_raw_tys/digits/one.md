[
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Form(
                FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
            ),
        ),
        Ok(
            RawTerm(`mnist_classifier::fermi::FermiMatchResult`),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Form(
                FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
            ),
        ),
        Ok(
            RawTerm(`core::option::Option mnist::MnistLabel`),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Form(
                FormPath(`mnist_classifier::digits::one::upmost`, `Fn`),
            ),
        ),
        Ok(
            RawTerm(`Fp(~ mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent) -> core::option::Option core::num::f32`),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Form(
                FormPath(`mnist_classifier::digits::one::downmost`, `Fn`),
            ),
        ),
        Ok(
            RawTerm(`Fp(~ mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent) -> core::option::Option core::num::f32`),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Form(
                FormPath(`mnist_classifier::digits::one::hat`, `Fn`),
            ),
        ),
        Ok(
            RawTerm(`Fp(~ mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent) -> core::option::Option core::num::f32`),
        ),
    ),
]