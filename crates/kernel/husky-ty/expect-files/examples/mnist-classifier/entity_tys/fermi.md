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
                                TermApplication(
                                    Id {
                                        value: 14,
                                    },
                                ),
                            ),
                        },
                        TermDurantParameter {
                            ty: Application(
                                TermApplication(
                                    Id {
                                        value: 15,
                                    },
                                ),
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