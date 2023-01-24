[
    (
        TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
        Err(
            Derived(
                DeclError,
            ),
        ),
    ),
    (
        TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
        Ok(
            Application(
                TermApplication {
                    function: Category(
                        Sort,
                    ),
                    argument: Universe(
                        TermUniverse(
                            1,
                        ),
                    ),
                },
            ),
        ),
    ),
    (
        FormPath(`mnist_classifier::line_segment_sketch::go_right`, `Function`),
        Ok(
            Durant(
                TermDurant {
                    kind: Fp,
                    params: [
                        TermDurantParameter {
                            ty: Entity(
                                TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                            ),
                        },
                        TermDurantParameter {
                            ty: Entity(
                                TypePath(`core::num::f32`, `Alien`),
                            ),
                        },
                    ],
                    y: Entity(
                        TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                    ),
                },
            ),
        ),
    ),
    (
        FormPath(`mnist_classifier::line_segment_sketch::go_left`, `Function`),
        Ok(
            Durant(
                TermDurant {
                    kind: Fp,
                    params: [
                        TermDurantParameter {
                            ty: Entity(
                                TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                            ),
                        },
                        TermDurantParameter {
                            ty: Entity(
                                TypePath(`core::num::f32`, `Alien`),
                            ),
                        },
                    ],
                    y: Entity(
                        TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                    ),
                },
            ),
        ),
    ),
    (
        FormPath(`mnist_classifier::line_segment_sketch::extend_end`, `Function`),
        Ok(
            Durant(
                TermDurant {
                    kind: Fp,
                    params: [
                        TermDurantParameter {
                            ty: Application(
                                TermApplication {
                                    function: Application(
                                        TermApplication {
                                            function: Entity(
                                                TypePath(`core::num::Ref`, `Alien`),
                                            ),
                                            argument: Literal(
                                                EvalLifetime,
                                            ),
                                        },
                                    ),
                                    argument: Entity(
                                        TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                    ),
                                },
                            ),
                        },
                        TermDurantParameter {
                            ty: Entity(
                                TypePath(`core::num::i32`, `Alien`),
                            ),
                        },
                        TermDurantParameter {
                            ty: Entity(
                                TypePath(`core::num::f32`, `Alien`),
                            ),
                        },
                    ],
                    y: Entity(
                        TypePath(`core::num::i32`, `Alien`),
                    ),
                },
            ),
        ),
    ),
    (
        FormPath(`mnist_classifier::line_segment_sketch::extend_start`, `Function`),
        Ok(
            Durant(
                TermDurant {
                    kind: Fp,
                    params: [
                        TermDurantParameter {
                            ty: Application(
                                TermApplication {
                                    function: Application(
                                        TermApplication {
                                            function: Entity(
                                                TypePath(`core::num::Ref`, `Alien`),
                                            ),
                                            argument: Literal(
                                                EvalLifetime,
                                            ),
                                        },
                                    ),
                                    argument: Entity(
                                        TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                    ),
                                },
                            ),
                        },
                        TermDurantParameter {
                            ty: Entity(
                                TypePath(`core::num::i32`, `Alien`),
                            ),
                        },
                        TermDurantParameter {
                            ty: Entity(
                                TypePath(`core::num::i32`, `Alien`),
                            ),
                        },
                        TermDurantParameter {
                            ty: Entity(
                                TypePath(`core::num::f32`, `Alien`),
                            ),
                        },
                    ],
                    y: Entity(
                        TypePath(`core::num::i32`, `Alien`),
                    ),
                },
            ),
        ),
    ),
    (
        FormPath(`mnist_classifier::line_segment_sketch::find_line_segments`, `Function`),
        Ok(
            Durant(
                TermDurant {
                    kind: Fp,
                    params: [
                        TermDurantParameter {
                            ty: Application(
                                TermApplication {
                                    function: Application(
                                        TermApplication {
                                            function: Entity(
                                                TypePath(`core::num::Ref`, `Alien`),
                                            ),
                                            argument: Literal(
                                                EvalLifetime,
                                            ),
                                        },
                                    ),
                                    argument: Entity(
                                        TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                    ),
                                },
                            ),
                        },
                        TermDurantParameter {
                            ty: Entity(
                                TypePath(`core::num::f32`, `Alien`),
                            ),
                        },
                    ],
                    y: Application(
                        TermApplication {
                            function: Entity(
                                TypePath(`core::vec::Vec`, `Alien`),
                            ),
                            argument: Entity(
                                TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                            ),
                        },
                    ),
                },
            ),
        ),
    ),
]