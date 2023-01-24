[
    (
        FormPath(`mnist_classifier::digits::nine::nine_match`, `Feature`),
        Err(
            Derived(
                SignatureError,
            ),
        ),
    ),
    (
        FormPath(`mnist_classifier::digits::nine::nine_match_refine`, `Feature`),
        Err(
            Derived(
                SignatureError,
            ),
        ),
    ),
    (
        FormPath(`mnist_classifier::digits::nine::is_nine`, `Feature`),
        Err(
            Derived(
                SignatureError,
            ),
        ),
    ),
    (
        FormPath(`mnist_classifier::digits::nine::downmost`, `Function`),
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
        FormPath(`mnist_classifier::digits::nine::big_cc`, `Function`),
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
]