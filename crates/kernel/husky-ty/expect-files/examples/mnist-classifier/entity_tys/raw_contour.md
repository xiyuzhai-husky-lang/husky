[
    (
        TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
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
        TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
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
        FormPath(`mnist_classifier::raw_contour::get_pixel_pair`, `Function`),
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
                                                value: 15,
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
                    ],
                    y: Entity(
                        TypePath(`core::raw_bits::r32`, `Alien`),
                    ),
                },
            ),
        ),
    ),
    (
        FormPath(`mnist_classifier::raw_contour::get_pixel_to_the_left`, `Function`),
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
                                                value: 15,
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
                    ],
                    y: Entity(
                        TypePath(`core::raw_bits::r32`, `Alien`),
                    ),
                },
            ),
        ),
    ),
    (
        FormPath(`mnist_classifier::raw_contour::get_pixel_to_the_right`, `Function`),
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
                                                value: 15,
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
                    ],
                    y: Entity(
                        TypePath(`core::raw_bits::r32`, `Alien`),
                    ),
                },
            ),
        ),
    ),
    (
        FormPath(`mnist_classifier::raw_contour::get_inward_direction`, `Function`),
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
                                                value: 15,
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
                                                value: 15,
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
                    ],
                    y: Entity(
                        TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                    ),
                },
            ),
        ),
    ),
    (
        FormPath(`mnist_classifier::raw_contour::get_angle_change`, `Function`),
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
                                                value: 29,
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
                                                value: 29,
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
        FormPath(`mnist_classifier::raw_contour::get_outward_direction`, `Function`),
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
                                                value: 15,
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
                                                value: 15,
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
                                                value: 29,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                        },
                    ],
                    y: Entity(
                        TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                    ),
                },
            ),
        ),
    ),
    (
        TypePath(`mnist_classifier::raw_contour::StreakCache`, `Struct`),
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
        FormPath(`mnist_classifier::raw_contour::get_concave_middle_point`, `Function`),
        Ok(
            Durant(
                TermDurant {
                    kind: Fp,
                    params: [
                        TermDurantParameter {
                            ty: Application(
                                TermApplication(
                                    Id {
                                        value: 20,
                                    },
                                ),
                            ),
                        },
                    ],
                    y: Entity(
                        TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                    ),
                },
            ),
        ),
    ),
    (
        FormPath(`mnist_classifier::raw_contour::find_raw_contours`, `Function`),
        Ok(
            Durant(
                TermDurant {
                    kind: Fp,
                    params: [
                        TermDurantParameter {
                            ty: Application(
                                TermApplication(
                                    Id {
                                        value: 19,
                                    },
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
                                TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                            ),
                        },
                    ),
                },
            ),
        ),
    ),
]