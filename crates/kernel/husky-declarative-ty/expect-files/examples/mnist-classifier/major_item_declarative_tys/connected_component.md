[
    (
        ItemPath::MajorItem(
            MajarItemPath::Type(
                TypePath(`mnist_classifier::connected_component::ConnectedComponentDistribution`, `Struct`),
            ),
        ),
        Ok(
            DeclarativeTerm(`Type`),
        ),
    ),
    (
        ItemPath::MajorItem(
            MajarItemPath::Type(
                TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
            ),
        ),
        Ok(
            DeclarativeTerm(`Type`),
        ),
    ),
    (
        ItemPath::MajorItem(
            MajarItemPath::Fugitive(
                FugitivePath(`mnist_classifier::connected_component::hole_tmpl`, `Fn`),
            ),
        ),
        Ok(
            DeclarativeTerm(`fn(~ mnist_classifier::raw_contour::RawContour) -> core::option::Option core::num::f32`),
        ),
    ),
    (
        ItemPath::MajorItem(
            MajarItemPath::Type(
                TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
            ),
        ),
        Ok(
            DeclarativeTerm(`Type`),
        ),
    ),
    (
        ItemPath::MajorItem(
            MajarItemPath::Fugitive(
                FugitivePath(`mnist_classifier::connected_component::horizontal_extend`, `Fn`),
            ),
        ),
        Ok(
            DeclarativeTerm(`fn(core::raw_bits::r32, core::raw_bits::r32) -> core::raw_bits::r32`),
        ),
    ),
    (
        ItemPath::MajorItem(
            MajarItemPath::Fugitive(
                FugitivePath(`mnist_classifier::connected_component::find_connected_components`, `Fn`),
            ),
        ),
        Ok(
            DeclarativeTerm(`fn(mnist::BinaryImage28) -> [] mnist_classifier::connected_component::ConnectedComponent`),
        ),
    ),
]