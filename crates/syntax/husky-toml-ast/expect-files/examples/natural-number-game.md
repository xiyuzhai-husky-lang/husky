Ok(
    TomlAstSheet {
        expr_arena: Arena {
            data: [
                TomlExpr::String(
                    "natural-number-game",
                ),
                TomlExpr::String(
                    "0.1.0",
                ),
                TomlExpr::String(
                    "example natural number game",
                ),
                TomlExpr::String(
                    "MIT OR Apache-2.0",
                ),
                TomlExpr::String(
                    "0.1.0",
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
                            TomlSectionEntry {
                                line_group_idx: TomlLineGroupIdx(
                                    2,
                                ),
                                key: Word(
                                    Id {
                                        value: 5,
                                    },
                                ),
                                value: Some(
                                    1,
                                ),
                            },
                            TomlSectionEntry {
                                line_group_idx: TomlLineGroupIdx(
                                    3,
                                ),
                                key: Word(
                                    Id {
                                        value: 8,
                                    },
                                ),
                                value: Some(
                                    2,
                                ),
                            },
                            TomlSectionEntry {
                                line_group_idx: TomlLineGroupIdx(
                                    4,
                                ),
                                key: Word(
                                    Id {
                                        value: 6,
                                    },
                                ),
                                value: Some(
                                    3,
                                ),
                            },
                        ],
                    },
                    TomlSection {
                        title: TomlSectionTitle(
                            [
                                Word(
                                    Id {
                                        value: 19,
                                    },
                                ),
                            ],
                        ),
                        kind: TomlSectionKind::Normal,
                        entries: [
                            TomlSectionEntry {
                                line_group_idx: TomlLineGroupIdx(
                                    6,
                                ),
                                key: Word(
                                    Id {
                                        value: 21,
                                    },
                                ),
                                value: Some(
                                    4,
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
            TomlLineGroup::KeyValue(
                Word(
                    "version",
                ),
                Some(
                    1,
                ),
            ),
            TomlLineGroup::KeyValue(
                Word(
                    "description",
                ),
                Some(
                    2,
                ),
            ),
            TomlLineGroup::KeyValue(
                Word(
                    "license",
                ),
                Some(
                    3,
                ),
            ),
            TomlLineGroup::SectionTitle {
                title: [
                    Word(
                        "dependencies",
                    ),
                ],
                kind: TomlSectionKind::Normal,
            },
            TomlLineGroup::KeyValue(
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
                        value: 3,
                    },
                ): Section(
                    0,
                ),
                Word(
                    Id {
                        value: 19,
                    },
                ): Section(
                    1,
                ),
            },
        },
    },
)