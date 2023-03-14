Ok(
    TomlAst {
        exprs: Arena {
            data: [
                String(
                    "mnist-classifier",
                ),
                String(
                    "0.1.0",
                ),
                String(
                    "example mnist classifier",
                ),
                String(
                    "MIT OR Apache-2.0",
                ),
                String(
                    "0.1.0",
                ),
            ],
        },
        sections: TomlSectionSheet {
            arena: Arena {
                data: [
                    TomlSection {
                        title: [
                            Word(
                                Id {
                                    value: 1,
                                },
                            ),
                        ],
                        kind: TomlSectionKind::Normal,
                        key_value_pairs: [
                            (
                                1,
                                Word(
                                    Id {
                                        value: 2,
                                    },
                                ),
                                Some(
                                    0,
                                ),
                            ),
                            (
                                2,
                                Word(
                                    Id {
                                        value: 3,
                                    },
                                ),
                                Some(
                                    1,
                                ),
                            ),
                            (
                                3,
                                Word(
                                    Id {
                                        value: 6,
                                    },
                                ),
                                Some(
                                    2,
                                ),
                            ),
                            (
                                4,
                                Word(
                                    Id {
                                        value: 4,
                                    },
                                ),
                                Some(
                                    3,
                                ),
                            ),
                        ],
                    },
                    TomlSection {
                        title: [
                            Word(
                                Id {
                                    value: 14,
                                },
                            ),
                        ],
                        kind: TomlSectionKind::Normal,
                        key_value_pairs: [
                            (
                                6,
                                Word(
                                    Id {
                                        value: 15,
                                    },
                                ),
                                Some(
                                    4,
                                ),
                            ),
                        ],
                    },
                ],
            },
            errors: [],
        },
        line_groups: [
            SectionTitle {
                title: [
                    Word(
                        Id {
                            value: 1,
                        },
                    ),
                ],
                kind: Normal,
            },
            KeyValue(
                Word(
                    Id {
                        value: 2,
                    },
                ),
                Some(
                    0,
                ),
            ),
            KeyValue(
                Word(
                    Id {
                        value: 3,
                    },
                ),
                Some(
                    1,
                ),
            ),
            KeyValue(
                Word(
                    Id {
                        value: 6,
                    },
                ),
                Some(
                    2,
                ),
            ),
            KeyValue(
                Word(
                    Id {
                        value: 4,
                    },
                ),
                Some(
                    3,
                ),
            ),
            SectionTitle {
                title: [
                    Word(
                        Id {
                            value: 14,
                        },
                    ),
                ],
                kind: Normal,
            },
            KeyValue(
                Word(
                    Id {
                        value: 15,
                    },
                ),
                Some(
                    4,
                ),
            ),
        ],
        table: TomlTable {
            data: {
                Word(
                    Id {
                        value: 1,
                    },
                ): Section(
                    0,
                ),
                Word(
                    Id {
                        value: 14,
                    },
                ): Section(
                    1,
                ),
            },
        },
    },
)