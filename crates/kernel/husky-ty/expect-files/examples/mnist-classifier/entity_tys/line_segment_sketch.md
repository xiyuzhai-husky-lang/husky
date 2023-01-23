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
                    m: Category(
                        Sort,
                    ),
                    n: Universe(
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
                                ModuleItem(
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 33,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                        },
                        TermDurantParameter {
                            ty: Entity(
                                ModuleItem(
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 13,
                                            },
                                        ),
                                    ),
                                ),
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
                                ModuleItem(
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 33,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                        },
                        TermDurantParameter {
                            ty: Entity(
                                ModuleItem(
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 13,
                                            },
                                        ),
                                    ),
                                ),
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
                                TermApplication(
                                    Id {
                                        value: 4,
                                    },
                                ),
                            ),
                        },
                        TermDurantParameter {
                            ty: Entity(
                                ModuleItem(
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 9,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                        },
                        TermDurantParameter {
                            ty: Entity(
                                ModuleItem(
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 13,
                                            },
                                        ),
                                    ),
                                ),
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
                                TermApplication(
                                    Id {
                                        value: 4,
                                    },
                                ),
                            ),
                        },
                        TermDurantParameter {
                            ty: Entity(
                                ModuleItem(
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 9,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                        },
                        TermDurantParameter {
                            ty: Entity(
                                ModuleItem(
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 9,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                        },
                        TermDurantParameter {
                            ty: Entity(
                                ModuleItem(
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 13,
                                            },
                                        ),
                                    ),
                                ),
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
                                TermApplication(
                                    Id {
                                        value: 4,
                                    },
                                ),
                            ),
                        },
                        TermDurantParameter {
                            ty: Entity(
                                ModuleItem(
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 13,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                        },
                    ],
                    y: Application(
                        TermApplication {
                            m: Entity(
                                TypePath(`core::vec::Vec`, `Alien`),
                            ),
                            n: Entity(
                                TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                            ),
                        },
                    ),
                },
            ),
        ),
    ),
]