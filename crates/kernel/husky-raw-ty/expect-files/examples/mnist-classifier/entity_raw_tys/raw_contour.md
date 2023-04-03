[
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Type(
                TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
            ),
        ),
        Ok(
            RawTerm(`Type`),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Type(
                TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
            ),
        ),
        Ok(
            RawTerm(`Type`),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Form(
                FormPath(`mnist_classifier::raw_contour::get_pixel_pair`, `Fn`),
            ),
        ),
        Ok(
            RawTerm(`Fp(core::raw_bits::r32, core::num::i32) -> core::raw_bits::r32`),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Form(
                FormPath(`mnist_classifier::raw_contour::get_pixel_to_the_left`, `Fn`),
            ),
        ),
        Ok(
            RawTerm(`Fp(core::raw_bits::r32, core::num::i32) -> core::raw_bits::r32`),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Form(
                FormPath(`mnist_classifier::raw_contour::get_pixel_to_the_right`, `Fn`),
            ),
        ),
        Ok(
            RawTerm(`Fp(core::raw_bits::r32, core::num::i32) -> core::raw_bits::r32`),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Form(
                FormPath(`mnist_classifier::raw_contour::get_inward_direction`, `Fn`),
            ),
        ),
        Ok(
            RawTerm(`Fp(core::raw_bits::r32, core::raw_bits::r32, core::num::i32) -> mnist_classifier::raw_contour::Direction`),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Form(
                FormPath(`mnist_classifier::raw_contour::get_angle_change`, `Fn`),
            ),
        ),
        Ok(
            RawTerm(`Fp(mnist_classifier::raw_contour::Direction, mnist_classifier::raw_contour::Direction) -> core::num::i32`),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Form(
                FormPath(`mnist_classifier::raw_contour::get_outward_direction`, `Fn`),
            ),
        ),
        Ok(
            RawTerm(`Fp(core::raw_bits::r32, core::raw_bits::r32, core::num::i32, mnist_classifier::raw_contour::Direction) -> mnist_classifier::raw_contour::Direction`),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Type(
                TypePath(`mnist_classifier::raw_contour::StreakCache`, `Struct`),
            ),
        ),
        Ok(
            RawTerm(`Type`),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Form(
                FormPath(`mnist_classifier::raw_contour::get_concave_middle_point`, `Fn`),
            ),
        ),
        Ok(
            RawTerm(`Fp([] mnist_classifier::geom2d::Point2d) -> mnist_classifier::geom2d::Point2d`),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Form(
                FormPath(`mnist_classifier::raw_contour::find_raw_contours`, `Fn`),
            ),
        ),
        Ok(
            RawTerm(`Fp(core::mem::Ref mnist_classifier::connected_component::ConnectedComponent) -> [] mnist_classifier::raw_contour::RawContour`),
        ),
    ),
]