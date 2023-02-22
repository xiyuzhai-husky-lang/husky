[
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Type(
                TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
            ),
        ),
        Ok(
            Term(`Type`),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Type(
                TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
            ),
        ),
        Ok(
            Term(`Type`),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Form(
                FormPath(`mnist_classifier::raw_contour::get_pixel_pair`, `Function`),
            ),
        ),
        Ok(
            Term(`Fp(r32, i32) -> r32`),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Form(
                FormPath(`mnist_classifier::raw_contour::get_pixel_to_the_left`, `Function`),
            ),
        ),
        Ok(
            Term(`Fp(r32, i32) -> r32`),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Form(
                FormPath(`mnist_classifier::raw_contour::get_pixel_to_the_right`, `Function`),
            ),
        ),
        Ok(
            Term(`Fp(r32, i32) -> r32`),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Form(
                FormPath(`mnist_classifier::raw_contour::get_inward_direction`, `Function`),
            ),
        ),
        Ok(
            Term(`Fp(r32, r32, i32) -> Direction`),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Form(
                FormPath(`mnist_classifier::raw_contour::get_angle_change`, `Function`),
            ),
        ),
        Ok(
            Term(`Fp(Direction, Direction) -> i32`),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Form(
                FormPath(`mnist_classifier::raw_contour::get_outward_direction`, `Function`),
            ),
        ),
        Ok(
            Term(`Fp(r32, r32, i32, Direction) -> Direction`),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Type(
                TypePath(`mnist_classifier::raw_contour::StreakCache`, `Struct`),
            ),
        ),
        Ok(
            Term(`Type`),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Form(
                FormPath(`mnist_classifier::raw_contour::get_concave_middle_point`, `Function`),
            ),
        ),
        Ok(
            Term(`Fp(List Point2d) -> Point2d`),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Form(
                FormPath(`mnist_classifier::raw_contour::find_raw_contours`, `Function`),
            ),
        ),
        Ok(
            Term(`Fp(Ref 'eval ConnectedComponent) -> List RawContour`),
        ),
    ),
]