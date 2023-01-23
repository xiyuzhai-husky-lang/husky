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
                                TermApplication(
                                    Id {
                                        value: 4,
                                    },
                                ),
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