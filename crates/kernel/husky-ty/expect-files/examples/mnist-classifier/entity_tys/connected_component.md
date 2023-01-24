[
    (
        TypePath(`mnist_classifier::connected_component::ConnectedComponentDistribution`, `Struct`),
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
        TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
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
        FormPath(`mnist_classifier::connected_component::hole_tmpl`, `Function`),
        Ok(
            Durant(
                TermDurant {
                    kind: Fp,
                    params: [
                        TermDurantParameter {
                            ty: Application(
                                TermApplication {
                                    m: Application(
                                        TermApplication {
                                            m: Entity(
                                                TypePath(`core::num::Ref`, `Alien`),
                                            ),
                                            n: Literal(
                                                EvalLifetime,
                                            ),
                                        },
                                    ),
                                    n: Entity(
                                        TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                    ),
                                },
                            ),
                        },
                    ],
                    y: Application(
                        TermApplication {
                            m: Entity(
                                TypePath(`core::option::Option`, `Enum`),
                            ),
                            n: Entity(
                                TypePath(`core::num::f32`, `Alien`),
                            ),
                        },
                    ),
                },
            ),
        ),
    ),
    (
        TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
        Err(
            Derived(
                SignatureError,
            ),
        ),
    ),
    (
        FormPath(`mnist_classifier::connected_component::horizontal_extend`, `Function`),
        Ok(
            Durant(
                TermDurant {
                    kind: Fp,
                    params: [
                        TermDurantParameter {
                            ty: Entity(
                                TypePath(`core::raw_bits::r32`, `Alien`),
                            ),
                        },
                        TermDurantParameter {
                            ty: Entity(
                                TypePath(`core::raw_bits::r32`, `Alien`),
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
        FormPath(`mnist_classifier::connected_component::find_connected_components`, `Function`),
        Err(
            Derived(
                SignatureError,
            ),
        ),
    ),
]