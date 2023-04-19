[
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Form(
                FugitivePath(`mnist_classifier::digits::three::three_fermi_match`, `Val`),
            ),
        ),
        Ok(
            DeclarativeTerm(`mnist_classifier::fermi::FermiMatchResult`),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Form(
                FugitivePath(`mnist_classifier::digits::three::is_three`, `Val`),
            ),
        ),
        Ok(
            DeclarativeTerm(`core::option::Option mnist::MnistLabel`),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Form(
                FugitivePath(`mnist_classifier::digits::three::uparc`, `Fn`),
            ),
        ),
        Ok(
            DeclarativeTerm(`Fp(~ mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent) -> core::option::Option core::num::f32`),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Form(
                FugitivePath(`mnist_classifier::digits::three::downarc`, `Fn`),
            ),
        ),
        Ok(
            DeclarativeTerm(`Fp(~ mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent) -> core::option::Option core::num::f32`),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Form(
                FugitivePath(`mnist_classifier::digits::three::back`, `Fn`),
            ),
        ),
        Ok(
            DeclarativeTerm(`Fp(~ mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent) -> core::option::Option core::num::f32`),
        ),
    ),
]