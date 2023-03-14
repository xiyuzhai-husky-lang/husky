Ok(
    TomlAst {
        exprs: Arena {
            data: [
                TomlExpr::String(
                    "std",
                ),
                TomlExpr::String(
                    "0.0.0",
                ),
                TomlExpr::String(
                    "MIT OR Apache-2.0",
                ),
                TomlExpr::String(
                    "https://github.com/xiyuzhai-husky-lang/husky.git",
                ),
                TomlExpr::String(
                    "The Husky Standard Library",
                ),
                TomlExpr::Boolean(
                    false,
                ),
                TomlExpr::Boolean(
                    false,
                ),
                TomlExpr::String(
                    "2021",
                ),
                TomlExpr::Boolean(
                    false,
                ),
                TomlExpr::Boolean(
                    false,
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
                                        value: 4,
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
                                        value: 5,
                                    },
                                ),
                                Some(
                                    3,
                                ),
                            ),
                            (
                                5,
                                Word(
                                    Id {
                                        value: 6,
                                    },
                                ),
                                Some(
                                    4,
                                ),
                            ),
                            (
                                6,
                                Word(
                                    Id {
                                        value: 7,
                                    },
                                ),
                                Some(
                                    5,
                                ),
                            ),
                            (
                                7,
                                Word(
                                    Id {
                                        value: 9,
                                    },
                                ),
                                Some(
                                    6,
                                ),
                            ),
                            (
                                10,
                                Word(
                                    Id {
                                        value: 10,
                                    },
                                ),
                                Some(
                                    7,
                                ),
                            ),
                        ],
                    },
                    TomlSection {
                        title: [
                            Word(
                                Id {
                                    value: 11,
                                },
                            ),
                        ],
                        kind: TomlSectionKind::Normal,
                        key_value_pairs: [
                            (
                                12,
                                Word(
                                    Id {
                                        value: 12,
                                    },
                                ),
                                Some(
                                    8,
                                ),
                            ),
                            (
                                13,
                                Word(
                                    Id {
                                        value: 13,
                                    },
                                ),
                                Some(
                                    9,
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
                        value: 4,
                    },
                ),
                Some(
                    2,
                ),
            ),
            KeyValue(
                Word(
                    Id {
                        value: 5,
                    },
                ),
                Some(
                    3,
                ),
            ),
            KeyValue(
                Word(
                    Id {
                        value: 6,
                    },
                ),
                Some(
                    4,
                ),
            ),
            KeyValue(
                Word(
                    Id {
                        value: 7,
                    },
                ),
                Some(
                    5,
                ),
            ),
            KeyValue(
                Word(
                    Id {
                        value: 9,
                    },
                ),
                Some(
                    6,
                ),
            ),
            Comment,
            Comment,
            KeyValue(
                Word(
                    Id {
                        value: 10,
                    },
                ),
                Some(
                    7,
                ),
            ),
            SectionTitle {
                title: [
                    Word(
                        Id {
                            value: 11,
                        },
                    ),
                ],
                kind: Normal,
            },
            KeyValue(
                Word(
                    Id {
                        value: 12,
                    },
                ),
                Some(
                    8,
                ),
            ),
            KeyValue(
                Word(
                    Id {
                        value: 13,
                    },
                ),
                Some(
                    9,
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
                        value: 11,
                    },
                ): Section(
                    1,
                ),
            },
        },
    },
)