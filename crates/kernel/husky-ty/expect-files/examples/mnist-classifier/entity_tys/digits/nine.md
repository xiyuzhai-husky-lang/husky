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
                                TermApplication(
                                    Id {
                                        value: 9,
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
        FormPath(`mnist_classifier::digits::nine::big_cc`, `Function`),
        Ok(
            Durant(
                TermDurant {
                    kind: Fp,
                    params: [
                        TermDurantParameter {
                            ty: Application(
                                TermApplication(
                                    Id {
                                        value: 9,
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
]