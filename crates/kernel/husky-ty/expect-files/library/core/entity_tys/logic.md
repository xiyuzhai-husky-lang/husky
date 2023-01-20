[
    (
        TypePath(`core::logic::Prop`, `Alien`),
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
        TypePath(`core::logic::LogicAnd`, `Structure`),
        Ok(
            Curry(
                TermCurry {
                    x: Application(
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
                    y: Curry(
                        TermCurry {
                            x: Application(
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
                            y: Application(
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
                        },
                    ),
                },
            ),
        ),
    ),
    (
        TypePath(`core::logic::LogicOr`, `Inductive`),
        Ok(
            Curry(
                TermCurry {
                    x: Application(
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
                    y: Curry(
                        TermCurry {
                            x: Application(
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
                            y: Application(
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
                        },
                    ),
                },
            ),
        ),
    ),
]