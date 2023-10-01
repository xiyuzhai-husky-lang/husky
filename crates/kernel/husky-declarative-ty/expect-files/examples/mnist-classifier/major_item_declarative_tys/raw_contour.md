[
    (
        ItemPath::MajorItem(
            MajorItemPath::Type(
                TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
            ),
        ),
        Ok(
            DeclarativeTerm(`Type`),
        ),
    ),
    (
        ItemPath::MajorItem(
            MajorItemPath::Type(
                TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
            ),
        ),
        Ok(
            DeclarativeTerm(`Type`),
        ),
    ),
    (
        ItemPath::MajorItem(
            MajorItemPath::Fugitive(
                FugitivePath(`mnist_classifier::raw_contour::get_pixel_pair`, `FunctionFn`),
            ),
        ),
        Ok(
            DeclarativeTerm(`fn(core::raw_bits::r32, core::num::i32) -> core::raw_bits::r32`),
        ),
    ),
    (
        ItemPath::MajorItem(
            MajorItemPath::Fugitive(
                FugitivePath(`mnist_classifier::raw_contour::get_pixel_to_the_left`, `FunctionFn`),
            ),
        ),
        Ok(
            DeclarativeTerm(`fn(core::raw_bits::r32, core::num::i32) -> core::raw_bits::r32`),
        ),
    ),
    (
        ItemPath::MajorItem(
            MajorItemPath::Fugitive(
                FugitivePath(`mnist_classifier::raw_contour::get_pixel_to_the_right`, `FunctionFn`),
            ),
        ),
        Ok(
            DeclarativeTerm(`fn(core::raw_bits::r32, core::num::i32) -> core::raw_bits::r32`),
        ),
    ),
    (
        ItemPath::MajorItem(
            MajorItemPath::Fugitive(
                FugitivePath(`mnist_classifier::raw_contour::get_inward_direction`, `FunctionFn`),
            ),
        ),
        Ok(
            DeclarativeTerm(`fn(core::raw_bits::r32, core::raw_bits::r32, core::num::i32) -> mnist_classifier::raw_contour::Direction`),
        ),
    ),
    (
        ItemPath::MajorItem(
            MajorItemPath::Fugitive(
                FugitivePath(`mnist_classifier::raw_contour::get_angle_change`, `FunctionFn`),
            ),
        ),
        Ok(
            DeclarativeTerm(`fn(mnist_classifier::raw_contour::Direction, mnist_classifier::raw_contour::Direction) -> core::num::i32`),
        ),
    ),
    (
        ItemPath::MajorItem(
            MajorItemPath::Fugitive(
                FugitivePath(`mnist_classifier::raw_contour::get_outward_direction`, `FunctionFn`),
            ),
        ),
        Ok(
            DeclarativeTerm(`fn(core::raw_bits::r32, core::raw_bits::r32, core::num::i32, mnist_classifier::raw_contour::Direction) -> mnist_classifier::raw_contour::Direction`),
        ),
    ),
    (
        ItemPath::MajorItem(
            MajorItemPath::Type(
                TypePath(`mnist_classifier::raw_contour::StreakCache`, `Struct`),
            ),
        ),
        Ok(
            DeclarativeTerm(`Type`),
        ),
    ),
    (
        ItemPath::MajorItem(
            MajorItemPath::Fugitive(
                FugitivePath(`mnist_classifier::raw_contour::get_concave_middle_point`, `FunctionFn`),
            ),
        ),
        Ok(
            DeclarativeTerm(`fn([] mnist_classifier::geom2d::Point2d) -> mnist_classifier::geom2d::Point2d`),
        ),
    ),
    (
        ItemPath::MajorItem(
            MajorItemPath::Fugitive(
                FugitivePath(`mnist_classifier::raw_contour::find_raw_contours`, `FunctionFn`),
            ),
        ),
        Ok(
            DeclarativeTerm(`fn(~ mnist_classifier::connected_component::ConnectedComponent) -> [] mnist_classifier::raw_contour::RawContour`),
        ),
    ),
]