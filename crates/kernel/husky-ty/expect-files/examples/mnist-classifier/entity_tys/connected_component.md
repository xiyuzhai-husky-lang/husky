[
    (
        TypePath(`mnist_classifier::connected_component::ConnectedComponentDistribution`, `Struct`),
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
        TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
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
        FormPath(`mnist_classifier::connected_component::hole_tmpl`, `Function`),
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
                    ],
                    y: Application(
                        TermApplication {
                            function: Entity(
                                TypePath(`core::option::Option`, `Enum`),
                            ),
                            argument: Entity(
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