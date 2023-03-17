Ok(
    TomlAstSheet {
        expr_arena: Arena {
            data: [
                TomlExpr::String(
                    "quick-sort",
                ),
            ],
        },
        section_arena: TomlSectionAstSheet {
            arena: Arena {
                data: [
                    TomlSectionAst {
                        title: TomlSectionTitle {
                            words: [
                                Word(
                                    "package",
                                ),
                            ],
                        },
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