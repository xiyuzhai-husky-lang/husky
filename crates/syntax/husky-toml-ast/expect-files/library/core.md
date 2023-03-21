Ok(
    TomlAstSheet {
        expr_arena: Arena {
            data: [
                TomlExpr::String(
                    "core",
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
                    "The Husky Core Library",
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
                                        value: 6,
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
                                        value: 7,
                                    },
                                ),
                                value: Some(
                                    3,
                                ),
                            },
                            TomlSectionEntry {
                                line_group_idx: TomlLineGroupIdx(
                                    5,
                                ),
                                key: Word(
                                    Id {
                                        value: 8,
                                    },
                                ),
                                value: Some(
                                    4,
                                ),
                            },
                            TomlSectionEntry {
                                line_group_idx: TomlLineGroupIdx(
                                    6,
                                ),
                                key: Word(
                                    Id {
                                        value: 9,
                                    },
                                ),
                                value: Some(
                                    5,
                                ),
                            },
                            TomlSectionEntry {
                                line_group_idx: TomlLineGroupIdx(
                                    7,
                                ),
                                key: Word(
                                    Id {
                                        value: 11,
                                    },
                                ),
                                value: Some(
                                    6,
                                ),
                            },
                            TomlSectionEntry {
                                line_group_idx: TomlLineGroupIdx(
                                    10,
                                ),
                                key: Word(
                                    Id {
                                        value: 12,
                                    },
                                ),
                                value: Some(
                                    7,
                                ),
                            },
                        ],
                    },
                    TomlSection {
                        title: TomlSectionTitle(
                            [
                                Word(
                                    Id {
                                        value: 13,
                                    },
                                ),
                            ],
                        ),
                        kind: TomlSectionKind::Normal,
                        entries: [
                            TomlSectionEntry {
                                line_group_idx: TomlLineGroupIdx(
                                    12,
                                ),
                                key: Word(
                                    Id {
                                        value: 14,
                                    },
                                ),
                                value: Some(
                                    8,
                                ),
                            },
                            TomlSectionEntry {
                                line_group_idx: TomlLineGroupIdx(
                                    13,
                                ),
                                key: Word(
                                    Id {
                                        value: 15,
                                    },
                                ),
                                value: Some(
                                    9,
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
                    "license",
                ),
                Some(
                    2,
                ),
            ),
            TomlLineGroup::KeyValue(
                Word(
                    "repository",
                ),
                Some(
                    3,
                ),
            ),
            TomlLineGroup::KeyValue(
                Word(
                    "description",
                ),
                Some(
                    4,
                ),
            ),
            TomlLineGroup::KeyValue(
                Word(
                    "autotests",
                ),
                Some(
                    5,
                ),
            ),
            TomlLineGroup::KeyValue(
                Word(
                    "autobenches",
                ),
                Some(
                    6,
                ),
            ),
            TomlLineGroup::Comment,
            TomlLineGroup::Comment,
            TomlLineGroup::KeyValue(
                Word(
                    "edition",
                ),
                Some(
                    7,
                ),
            ),
            TomlLineGroup::SectionTitle {
                title: [
                    Word(
                        "lib",
                    ),
                ],
                kind: TomlSectionKind::Normal,
            },
            TomlLineGroup::KeyValue(
                Word(
                    "test",
                ),
                Some(
                    8,
                ),
            ),
            TomlLineGroup::KeyValue(
                Word(
                    "bench",
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
                        value: 3,
                    },
                ): Section(
                    0,
                ),
                Word(
                    Id {
                        value: 13,
                    },
                ): Section(
                    1,
                ),
            },
        },
    },
)