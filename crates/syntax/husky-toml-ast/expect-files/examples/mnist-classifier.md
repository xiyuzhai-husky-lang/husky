Ok(
    TomlAst {
        exprs: Arena {
            data: [
                TomlExpr::String(
                    "mnist-classifier",
                ),
                TomlExpr::String(
                    "0.1.0",
                ),
                TomlExpr::String(
                    "example mnist classifier",
                ),
                TomlExpr::String(
                    "MIT OR Apache-2.0",
                ),
                TomlExpr::String(
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
                                "package",
                            ),
                        ],
                        kind: TomlSectionKind::Normal,
                        key_value_pairs: [
                            (
                                1,
                                Word(
                                    "name",
                                ),
                                Some(
                                    0,
                                ),
                            ),
                            (
                                2,
                                Word(
                                    "version",
                                ),
                                Some(
                                    1,
                                ),
                            ),
                            (
                                3,
                                Word(
                                    "description",
                                ),
                                Some(
                                    2,
                                ),
                            ),
                            (
                                4,
                                Word(
                                    "license",
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
                                "dependencies",
                            ),
                        ],
                        kind: TomlSectionKind::Normal,
                        key_value_pairs: [
                            (
                                6,
                                Word(
                                    "mnist",
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
            TomlGroup::SectionTitle {
                title: [
                    Word(
                        "package",
                    ),
                ],
                kind: TomlSectionKind::Normal,
            },
            TomlGroup::KeyValue(
                Word(
                    "name",
                ),
                Some(
                    0,
                ),
            ),
            TomlGroup::KeyValue(
                Word(
                    "version",
                ),
                Some(
                    1,
                ),
            ),
            TomlGroup::KeyValue(
                Word(
                    "description",
                ),
                Some(
                    2,
                ),
            ),
            TomlGroup::KeyValue(
                Word(
                    "license",
                ),
                Some(
                    3,
                ),
            ),
            TomlGroup::SectionTitle {
                title: [
                    Word(
                        "dependencies",
                    ),
                ],
                kind: TomlSectionKind::Normal,
            },
            TomlGroup::KeyValue(
                Word(
                    "mnist",
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