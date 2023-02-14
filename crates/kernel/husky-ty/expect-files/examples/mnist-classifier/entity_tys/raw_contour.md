[
    (
        TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
        Ok(
            Term(`Type`),
        ),
    ),
    (
        TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
        Ok(
            Term(`Type`),
        ),
    ),
    (
        FormPath(`mnist_classifier::raw_contour::get_pixel_pair`, `Function`),
        Ok(
            Term(`Fp(r32, i32) -> r32`),
        ),
    ),
    (
        FormPath(`mnist_classifier::raw_contour::get_pixel_to_the_left`, `Function`),
        Ok(
            Term(`Fp(r32, i32) -> r32`),
        ),
    ),
    (
        FormPath(`mnist_classifier::raw_contour::get_pixel_to_the_right`, `Function`),
        Ok(
            Term(`Fp(r32, i32) -> r32`),
        ),
    ),
    (
        FormPath(`mnist_classifier::raw_contour::get_inward_direction`, `Function`),
        Ok(
            Term(`Fp(r32, r32, i32) -> Direction`),
        ),
    ),
    (
        FormPath(`mnist_classifier::raw_contour::get_angle_change`, `Function`),
        Ok(
            Term(`Fp(Direction, Direction) -> i32`),
        ),
    ),
    (
        FormPath(`mnist_classifier::raw_contour::get_outward_direction`, `Function`),
        Ok(
            Term(`Fp(r32, r32, i32, Direction) -> Direction`),
        ),
    ),
    (
        TypePath(`mnist_classifier::raw_contour::StreakCache`, `Struct`),
        Ok(
            Term(`Type`),
        ),
    ),
    (
        FormPath(`mnist_classifier::raw_contour::get_concave_middle_point`, `Function`),
        Ok(
            Term(`Fp(List Point2d) -> Point2d`),
        ),
    ),
    (
        FormPath(`mnist_classifier::raw_contour::find_raw_contours`, `Function`),
        Ok(
            Term(`Fp(Ref TermLiteral::EvalLifetime ConnectedComponent) -> List RawContour`),
        ),
    ),
]