[
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Type(
                TypePath(`mnist_classifier::connected_component::ConnectedComponentDistribution`, `Struct`),
            ),
        ),
        Ok(
            DeclarativeTerm(`Type`),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Type(
                TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
            ),
        ),
        Ok(
            DeclarativeTerm(`Type`),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Form(
                FugitivePath(`mnist_classifier::connected_component::hole_tmpl`, `Fn`),
            ),
        ),
        Ok(
            DeclarativeTerm(`Fp(~ mnist_classifier::raw_contour::RawContour) -> core::option::Option core::num::f32`),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Type(
                TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
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
                FugitivePath(`mnist_classifier::connected_component::horizontal_extend`, `Fn`),
            ),
        ),
        Ok(
            DeclarativeTerm(`Fp(core::raw_bits::r32, core::raw_bits::r32) -> core::raw_bits::r32`),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Form(
                FugitivePath(`mnist_classifier::connected_component::find_connected_components`, `Fn`),
            ),
        ),
        Err(
            DeclarativeTypeError::Derived(
                DerivedDeclarativeTypeError::SignatureError,
            ),
        ),
    ),
]