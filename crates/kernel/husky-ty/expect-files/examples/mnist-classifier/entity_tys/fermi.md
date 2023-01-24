[
    (
        TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
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
        FormPath(`mnist_classifier::fermi::fermi_match`, `Function`),
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
                                    argument: Application(
                                        TermApplication {
                                            function: Entity(
                                                TypePath(`core::vec::Vec`, `Alien`),
                                            ),
                                            argument: Entity(
                                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            ),
                                        },
                                    ),
                                },
                            ),
                        },
                        TermDurantParameter {
                            ty: Application(
                                TermApplication {
                                    function: Entity(
                                        TypePath(`core::vec::Vec`, `Alien`),
                                    ),
                                    argument: Curry(
                                        TermCurry {
                                            x: Application(
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
                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    ),
                                                },
                                            ),
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
                                },
                            ),
                        },
                    ],
                    y: Entity(
                        TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                    ),
                },
            ),
        ),
    ),
]