```rust
Ok(
    TomlAstSheet {
        expr_arena: Arena {
            data: [
                TomlExpr::String(
                    "malamute",
                ),
                TomlExpr::String(
                    "MIT OR Apache-2.0",
                ),
                TomlExpr::String(
                    "https://github.com/xiyuzhai-husky-lang/husky.git",
                ),
                TomlExpr::String(
                    "The Husky Malamute Library for Basic Machine Learning Algorithms",
                ),
                TomlExpr::Boolean(
                    false,
                ),
                TomlExpr::Boolean(
                    false,
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
                                Coword(
                                    Id {
                                        value: 2,
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
                                        value: 3,
                                    },
                                ),
                                value: Some(
                                    0,
                                ),
                            },
                            TomlSectionEntry {
                                line_group_idx: TomlLineGroupIdx(
                                    3,
                                ),
                                key: Coword(
                                    Id {
                                        value: 7,
                                    },
                                ),
                                value: Some(
                                    1,
                                ),
                            },
                            TomlSectionEntry {
                                line_group_idx: TomlLineGroupIdx(
                                    4,
                                ),
                                key: Coword(
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
                                    5,
                                ),
                                key: Coword(
                                    Id {
                                        value: 9,
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
                                Coword(
                                    Id {
                                        value: 10,
                                    },
                                ),
                            ],
                        ),
                        kind: TomlSectionKind::Normal,
                        entries: [
                            TomlSectionEntry {
                                line_group_idx: TomlLineGroupIdx(
                                    7,
                                ),
                                key: Coword(
                                    Id {
                                        value: 11,
                                    },
                                ),
                                value: Some(
                                    4,
                                ),
                            },
                            TomlSectionEntry {
                                line_group_idx: TomlLineGroupIdx(
                                    8,
                                ),
                                key: Coword(
                                    Id {
                                        value: 13,
                                    },
                                ),
                                value: Some(
                                    5,
                                ),
                            },
                        ],
                    },
                    TomlSection {
                        title: TomlSectionTitle(
                            [
                                Coword(
                                    Id {
                                        value: 14,
                                    },
                                ),
                            ],
                        ),
                        kind: TomlSectionKind::Normal,
                        entries: [
                            TomlSectionEntry {
                                line_group_idx: TomlLineGroupIdx(
                                    10,
                                ),
                                key: Coword(
                                    Id {
                                        value: 15,
                                    },
                                ),
                                value: Some(
                                    6,
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
                    Coword(
                        "package",
                    ),
                ],
                kind: TomlSectionKind::Normal,
            },
            TomlLineGroup::KeyValue(
                Coword(
                    "name",
                ),
                Some(
                    0,
                ),
            ),
            TomlLineGroup::Err,
            TomlLineGroup::KeyValue(
                Coword(
                    "license",
                ),
                Some(
                    1,
                ),
            ),
            TomlLineGroup::KeyValue(
                Coword(
                    "repository",
                ),
                Some(
                    2,
                ),
            ),
            TomlLineGroup::KeyValue(
                Coword(
                    "description",
                ),
                Some(
                    3,
                ),
            ),
            TomlLineGroup::SectionTitle {
                title: [
                    Coword(
                        "lib",
                    ),
                ],
                kind: TomlSectionKind::Normal,
            },
            TomlLineGroup::KeyValue(
                Coword(
                    "test",
                ),
                Some(
                    4,
                ),
            ),
            TomlLineGroup::KeyValue(
                Coword(
                    "bench",
                ),
                Some(
                    5,
                ),
            ),
            TomlLineGroup::SectionTitle {
                title: [
                    Coword(
                        "dependencies",
                    ),
                ],
                kind: TomlSectionKind::Normal,
            },
            TomlLineGroup::KeyValue(
                Coword(
                    "ml-task",
                ),
                Some(
                    6,
                ),
            ),
        ],
        table: TomlTable {
            data: {
                Coword(
                    Id {
                        value: 2,
                    },
                ): Section(
                    0,
                ),
                Coword(
                    Id {
                        value: 10,
                    },
                ): Section(
                    1,
                ),
                Coword(
                    Id {
                        value: 14,
                    },
                ): Section(
                    2,
                ),
            },
        },
    },
)
```