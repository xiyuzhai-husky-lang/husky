[
    (
        TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
        Ok(
            DeclarativeTerm(`fn((~ mnist_classifier::connected_component::ConnectedComponent, [] mnist_classifier::geom2d::Point2d) -> mnist_classifier::raw_contour::RawContour`),
        ),
    ),
    (
        TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
        Err(
            DeclarativeTypeError::Original(
                OriginalDeclarativeTypeError::EnumTypeHasNoConstructor,
            ),
        ),
    ),
    (
        TypePath(`mnist_classifier::raw_contour::StreakCache`, `Struct`),
        Ok(
            DeclarativeTerm(`fn((core::num::i32, core::num::i32) -> mnist_classifier::raw_contour::StreakCache`),
        ),
    ),
]