[
    (
        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
        Err(
            Derived(
                SignatureError,
            ),
        ),
    ),
    (
        FormPath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Function`),
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
                                        TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                    ),
                                },
                            ),
                        },
                    ],
                    y: Application(
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
        ),
    ),
]