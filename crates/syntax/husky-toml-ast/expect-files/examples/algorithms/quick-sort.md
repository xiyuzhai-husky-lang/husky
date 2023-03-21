Ok(
    TomlAstSheet {
        expr_arena: Arena {
            data: [
                TomlExpr::String(
                    "quick-sort",
                ),
            ],
        },
        section_sheet: TomlSectionSheet {
            arena: Arena {
                data: [
                    TomlSection {
                        title: TomlSectionTitle(
                            [
                                Word(
                                    Id {
                                        value: 3,
                                    },
                                ),
                            ],
                        ),
                        kind: TomlSectionKind::Normal,
                        entries: [
                            TomlSectionEntry {
                                line_group_idx: TomlLineGroupIdx(
                                    1,
                                ),
                                key: Word(
                                    Id {
                                        value: 4,
                                    },
                                ),
                                value: Some(
                                    0,
                                ),
                            },
                        ],
                    },
                ],
            },
            errors: [],
        },
        line_groups: [
            TomlLineGroup::SectionTitle {
                title: [
                    Word(
                        "package",
                    ),
                ],
                kind: TomlSectionKind::Normal,
            },
            TomlLineGroup::KeyValue(
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
                        value: 3,
                    },
                ): Section(
                    0,
                ),
            },
        },
    },
)