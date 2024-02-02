[
    (
        ItemPath::Submodule(
            Room32,
            SubmoduleItemPath(
                ItemPathId {
                    data: ItemPathData::SubmoduleItem(
                        SubmoduleItemPathData {
                            submodule_path: SubmodulePath(
                                `mnist_classifier::line_segment_sketch::concave_component`,
                            ),
                        },
                    ),
                },
            ),
        ),
        Ok(
            DecTerm(`core::basic::Module`),
        ),
    ),
    (
        ItemPath::Submodule(
            Room32,
            SubmoduleItemPath(
                ItemPathId {
                    data: ItemPathData::SubmoduleItem(
                        SubmoduleItemPathData {
                            submodule_path: SubmodulePath(
                                `mnist_classifier::line_segment_sketch::convex_component`,
                            ),
                        },
                    ),
                },
            ),
        ),
        Ok(
            DecTerm(`core::basic::Module`),
        ),
    ),
    (
        ItemPath::Submodule(
            Room32,
            SubmoduleItemPath(
                ItemPathId {
                    data: ItemPathData::SubmoduleItem(
                        SubmoduleItemPathData {
                            submodule_path: SubmodulePath(
                                `mnist_classifier::line_segment_sketch::convexity`,
                            ),
                        },
                    ),
                },
            ),
        ),
        Ok(
            DecTerm(`core::basic::Module`),
        ),
    ),
    (
        ItemPath::Submodule(
            Room32,
            SubmoduleItemPath(
                ItemPathId {
                    data: ItemPathData::SubmoduleItem(
                        SubmoduleItemPathData {
                            submodule_path: SubmodulePath(
                                `mnist_classifier::line_segment_sketch::line_segment`,
                            ),
                        },
                    ),
                },
            ),
        ),
        Ok(
            DecTerm(`core::basic::Module`),
        ),
    ),
    (
        ItemPath::MajorItem(
            MajorItemPath::Type(
                TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
            ),
        ),
        Ok(
            DecTerm(`Type`),
        ),
    ),
    (
        ItemPath::MajorItem(
            MajorItemPath::Type(
                TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
            ),
        ),
        Ok(
            DecTerm(`Type`),
        ),
    ),
    (
        ItemPath::MajorItem(
            MajorItemPath::Fugitive(
                FugitivePath(`mnist_classifier::line_segment_sketch::go_right`, `FunctionFn`),
            ),
        ),
        Ok(
            DecTerm(`fn((mnist_classifier::geom2d::Vector2d, core::num::f32) -> mnist_classifier::geom2d::Vector2d`),
        ),
    ),
    (
        ItemPath::MajorItem(
            MajorItemPath::Fugitive(
                FugitivePath(`mnist_classifier::line_segment_sketch::go_left`, `FunctionFn`),
            ),
        ),
        Ok(
            DecTerm(`fn((mnist_classifier::geom2d::Vector2d, core::num::f32) -> mnist_classifier::geom2d::Vector2d`),
        ),
    ),
    (
        ItemPath::MajorItem(
            MajorItemPath::Fugitive(
                FugitivePath(`mnist_classifier::line_segment_sketch::extend_end`, `FunctionFn`),
            ),
        ),
        Ok(
            DecTerm(`fn((~ mnist_classifier::raw_contour::RawContour, core::num::i32, core::num::f32) -> core::num::i32`),
        ),
    ),
    (
        ItemPath::MajorItem(
            MajorItemPath::Fugitive(
                FugitivePath(`mnist_classifier::line_segment_sketch::extend_start`, `FunctionFn`),
            ),
        ),
        Ok(
            DecTerm(`fn((~ mnist_classifier::raw_contour::RawContour, core::num::i32, core::num::i32, core::num::f32) -> core::num::i32`),
        ),
    ),
    (
        ItemPath::MajorItem(
            MajorItemPath::Fugitive(
                FugitivePath(`mnist_classifier::line_segment_sketch::find_line_segments`, `FunctionFn`),
            ),
        ),
        Ok(
            DecTerm(`fn((~ mnist_classifier::raw_contour::RawContour, core::num::f32) -> [] mnist_classifier::line_segment_sketch::LineSegmentStroke`),
        ),
    ),
]