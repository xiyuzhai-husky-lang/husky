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
                                Coword(
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
                                key: Coword(
                                    Id {
                                        value: 4,
                                    },
                                ),
                                value: Some(
                                    1,
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
                    1,
                ),
            ),
        ],
        table: TomlTable {
            data: {
                Coword(
                    Id {
                        value: 3,
                    },
                ): Section(
                    1,
                ),
            },
        },
    },
)