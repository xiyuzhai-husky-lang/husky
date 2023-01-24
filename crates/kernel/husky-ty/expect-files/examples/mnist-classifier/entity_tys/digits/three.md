[
    (
        FormPath(`mnist_classifier::digits::three::three_fermi_match`, `Feature`),
        Err(
            Derived(
                SignatureError,
            ),
        ),
    ),
    (
        FormPath(`mnist_classifier::digits::three::is_three`, `Feature`),
        Err(
            Derived(
                SignatureError,
            ),
        ),
    ),
    (
        FormPath(`mnist_classifier::digits::three::uparc`, `Function`),
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
                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
        FormPath(`mnist_classifier::digits::three::downarc`, `Function`),
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
                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
        FormPath(`mnist_classifier::digits::three::back`, `Function`),
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
                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
]