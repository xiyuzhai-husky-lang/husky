[
    (
        TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
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
        FormPath(`mnist_classifier::fermi::fermi_match`, `Function`),
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
                                    n: Application(
                                        TermApplication {
                                            m: Entity(
                                                TypePath(`core::vec::Vec`, `Alien`),
                                            ),
                                            n: Entity(
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
                                    m: Entity(
                                        TypePath(`core::vec::Vec`, `Alien`),
                                    ),
                                    n: Curry(
                                        TermCurry {
                                            x: Application(
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
                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    ),
                                                },
                                            ),
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