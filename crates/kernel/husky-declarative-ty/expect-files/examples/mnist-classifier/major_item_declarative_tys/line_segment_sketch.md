[
    (
        ItemPath::Submodule(
            SubmodulePath(
                `mnist_classifier::line_segment_sketch::concave_component`,
            ),
        ),
        Ok(
            DeclarativeTerm(`core::basic::Module`),
        ),
    ),
    (
        ItemPath::Submodule(
            SubmodulePath(
                `mnist_classifier::line_segment_sketch::convex_component`,
            ),
        ),
        Ok(
            DeclarativeTerm(`core::basic::Module`),
        ),
    ),
    (
        ItemPath::Submodule(
            SubmodulePath(
                `mnist_classifier::line_segment_sketch::convexity`,
            ),
        ),
        Ok(
            DeclarativeTerm(`core::basic::Module`),
        ),
    ),
    (
        ItemPath::Submodule(
            SubmodulePath(
                `mnist_classifier::line_segment_sketch::line_segment`,
            ),
        ),
        Ok(
            DeclarativeTerm(`core::basic::Module`),
        ),
    ),
    (
        ItemPath::ModuleItem(
            ModuleItemPath::Type(
                TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
            ),
        ),
        Ok(
            DeclarativeTerm(`Type`),
        ),
    ),
    (
        ItemPath::ModuleItem(
            ModuleItemPath::Type(
                TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
            ),
        ),
        Ok(
            DeclarativeTerm(`Type`),
        ),
    ),
    (
        ItemPath::ModuleItem(
            ModuleItemPath::Fugitive(
                FugitivePath(`mnist_classifier::line_segment_sketch::go_right`, `Fn`),
            ),
        ),
        Ok(
            DeclarativeTerm(`fn(mnist_classifier::geom2d::Vector2d, core::num::f32) -> mnist_classifier::geom2d::Vector2d`),
        ),
    ),
    (
        ItemPath::ModuleItem(
            ModuleItemPath::Fugitive(
                FugitivePath(`mnist_classifier::line_segment_sketch::go_left`, `Fn`),
            ),
        ),
        Ok(
            DeclarativeTerm(`fn(mnist_classifier::geom2d::Vector2d, core::num::f32) -> mnist_classifier::geom2d::Vector2d`),
        ),
    ),
    (
        ItemPath::ModuleItem(
            ModuleItemPath::Fugitive(
                FugitivePath(`mnist_classifier::line_segment_sketch::extend_end`, `Fn`),
            ),
        ),
        Ok(
            DeclarativeTerm(`fn(~ mnist_classifier::raw_contour::RawContour, core::num::i32, core::num::f32) -> core::num::i32`),
        ),
    ),
    (
        ItemPath::ModuleItem(
            ModuleItemPath::Fugitive(
                FugitivePath(`mnist_classifier::line_segment_sketch::extend_start`, `Fn`),
            ),
        ),
        Ok(
            DeclarativeTerm(`fn(~ mnist_classifier::raw_contour::RawContour, core::num::i32, core::num::i32, core::num::f32) -> core::num::i32`),
        ),
    ),
    (
        ItemPath::ModuleItem(
            ModuleItemPath::Fugitive(
                FugitivePath(`mnist_classifier::line_segment_sketch::find_line_segments`, `Fn`),
            ),
        ),
        Ok(
            DeclarativeTerm(`fn(~ mnist_classifier::raw_contour::RawContour, core::num::f32) -> [] mnist_classifier::line_segment_sketch::LineSegmentStroke`),
        ),
    ),
]