```rust
[
    (
        ItemPath(`mnist_classifier::major::connected_components`),
        Ok(
            ItemDecTemplate::MajorItem(
                MajorItemDecTemplate::Form(
                    MajorFormDecTemplate::Val(
                        MajorValDecTemplate {
                            return_ty: DecTerm::Application(
                                DecApplication {
                                    function: DecTerm::List(
                                        DecList {
                                            toolchain: Toolchain {
                                                data: ToolchainData::Local {
                                                    library_path: "../../../library",
                                                },
                                            },
                                            items: [],
                                        },
                                    ),
                                    argument: DecTerm::ItemPath(
                                        DecItemPath::Type(
                                            TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                        ),
                                    ),
                                },
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::major::major_connected_component`),
        Ok(
            ItemDecTemplate::MajorItem(
                MajorItemDecTemplate::Form(
                    MajorFormDecTemplate::Val(
                        MajorValDecTemplate {
                            return_ty: DecTerm::Application(
                                DecApplication {
                                    function: DecTerm::LeashOrBitNot(
                                        Toolchain {
                                            data: ToolchainData::Local {
                                                library_path: "../../../library",
                                            },
                                        },
                                    ),
                                    argument: DecTerm::ItemPath(
                                        DecItemPath::Type(
                                            TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                        ),
                                    ),
                                },
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`),
        Ok(
            ItemDecTemplate::MajorItem(
                MajorItemDecTemplate::Form(
                    MajorFormDecTemplate::Val(
                        MajorValDecTemplate {
                            return_ty: DecTerm::ItemPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::f32`, `Extern`),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::major::major_raw_contours`),
        Ok(
            ItemDecTemplate::MajorItem(
                MajorItemDecTemplate::Form(
                    MajorFormDecTemplate::Val(
                        MajorValDecTemplate {
                            return_ty: DecTerm::Application(
                                DecApplication {
                                    function: DecTerm::LeashOrBitNot(
                                        Toolchain {
                                            data: ToolchainData::Local {
                                                library_path: "../../../library",
                                            },
                                        },
                                    ),
                                    argument: DecTerm::Application(
                                        DecApplication {
                                            function: DecTerm::List(
                                                DecList {
                                                    toolchain: Toolchain {
                                                        data: ToolchainData::Local {
                                                            library_path: "../../../library",
                                                        },
                                                    },
                                                    items: [],
                                                },
                                            ),
                                            argument: DecTerm::ItemPath(
                                                DecItemPath::Type(
                                                    TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                ),
                                            ),
                                        },
                                    ),
                                },
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::major::major_raw_contour`),
        Ok(
            ItemDecTemplate::MajorItem(
                MajorItemDecTemplate::Form(
                    MajorFormDecTemplate::Val(
                        MajorValDecTemplate {
                            return_ty: DecTerm::Application(
                                DecApplication {
                                    function: DecTerm::LeashOrBitNot(
                                        Toolchain {
                                            data: ToolchainData::Local {
                                                library_path: "../../../library",
                                            },
                                        },
                                    ),
                                    argument: DecTerm::ItemPath(
                                        DecItemPath::Type(
                                            TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                        ),
                                    ),
                                },
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::major::major_line_segment_sketch`),
        Ok(
            ItemDecTemplate::MajorItem(
                MajorItemDecTemplate::Form(
                    MajorFormDecTemplate::Val(
                        MajorValDecTemplate {
                            return_ty: DecTerm::Application(
                                DecApplication {
                                    function: DecTerm::LeashOrBitNot(
                                        Toolchain {
                                            data: ToolchainData::Local {
                                                library_path: "../../../library",
                                            },
                                        },
                                    ),
                                    argument: DecTerm::ItemPath(
                                        DecItemPath::Type(
                                            TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                        ),
                                    ),
                                },
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::major::major_concave_components`),
        Ok(
            ItemDecTemplate::MajorItem(
                MajorItemDecTemplate::Form(
                    MajorFormDecTemplate::Val(
                        MajorValDecTemplate {
                            return_ty: DecTerm::Application(
                                DecApplication {
                                    function: DecTerm::LeashOrBitNot(
                                        Toolchain {
                                            data: ToolchainData::Local {
                                                library_path: "../../../library",
                                            },
                                        },
                                    ),
                                    argument: DecTerm::Application(
                                        DecApplication {
                                            function: DecTerm::List(
                                                DecList {
                                                    toolchain: Toolchain {
                                                        data: ToolchainData::Local {
                                                            library_path: "../../../library",
                                                        },
                                                    },
                                                    items: [],
                                                },
                                            ),
                                            argument: DecTerm::ItemPath(
                                                DecItemPath::Type(
                                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                ),
                                            ),
                                        },
                                    ),
                                },
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
]
```