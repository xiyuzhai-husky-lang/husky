Ok(
    TomlAst {
        exprs: Arena {
            data: [
                String(
                    "quick-sort",
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
            },
        },
    },
)