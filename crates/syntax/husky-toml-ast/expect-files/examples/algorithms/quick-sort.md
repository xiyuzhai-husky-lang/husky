Ok(
    TomlAst {
        exprs: Arena {
            data: [
                TomlExpr::String(
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
                                    "name",
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
            TomlGroup::SectionTitle {
                title: [
                    Word(
                        Id {
                            value: 1,
                        },
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