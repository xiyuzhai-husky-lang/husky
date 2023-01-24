[
    (
        FormPath(`mnist_classifier::digits::seven::simple_seven_match`, `Feature`),
        Err(
            Derived(
                SignatureError,
            ),
        ),
    ),
    (
        FormPath(`mnist_classifier::digits::seven::simple_leftdown_pattern`, `Function`),
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
                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
        FormPath(`mnist_classifier::digits::seven::special_seven_match`, `Feature`),
        Err(
            Derived(
                SignatureError,
            ),
        ),
    ),
    (
        FormPath(`mnist_classifier::digits::seven::leftupcc_pattern`, `Function`),
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
                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
        FormPath(`mnist_classifier::digits::seven::leftdowncc_pattern`, `Function`),
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
                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
        FormPath(`mnist_classifier::digits::seven::is_seven`, `Feature`),
        Err(
            Derived(
                SignatureError,
            ),
        ),
    ),
]