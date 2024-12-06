```rust
Ok(
    TomlAstSheet {
        expr_arena: Arena {
            data: [
                TomlExpr::String(
                    "std",
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
        section_sheet: TomlSectionSheet {
            arena: Arena {
                data: [
                    TomlSection {
                        title: TomlSectionTitle(
                            [
                                Coword(
                                    "package",
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
                                    "name",
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
                                    "license",
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
                                    "repository",
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
                                    "description",
                                ),
                                value: Some(
                                    3,
                                ),
                            },
                            TomlSectionEntry {
                                line_group_idx: TomlLineGroupIdx(
                                    6,
                                ),
                                key: Coword(
                                    "autotests",
                                ),
                                value: Some(
                                    4,
                                ),
                            },
                            TomlSectionEntry {
                                line_group_idx: TomlLineGroupIdx(
                                    7,
                                ),
                                key: Coword(
                                    "autobenches",
                                ),
                                value: Some(
                                    5,
                                ),
                            },
                            TomlSectionEntry {
                                line_group_idx: TomlLineGroupIdx(
                                    10,
                                ),
                                key: Coword(
                                    "edition",
                                ),
                                value: Some(
                                    6,
                                ),
                            },
                        ],
                    },
                    TomlSection {
                        title: TomlSectionTitle(
                            [
                                Coword(
                                    "lib",
                                ),
                            ],
                        ),
                        kind: TomlSectionKind::Normal,
                        entries: [
                            TomlSectionEntry {
                                line_group_idx: TomlLineGroupIdx(
                                    12,
                                ),
                                key: Coword(
                                    "test",
                                ),
                                value: Some(
                                    7,
                                ),
                            },
                            TomlSectionEntry {
                                line_group_idx: TomlLineGroupIdx(
                                    13,
                                ),
                                key: Coword(
                                    "bench",
                                ),
                                value: Some(
                                    8,
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
            TomlLineGroup::KeyValue(
                Coword(
                    "autotests",
                ),
                Some(
                    4,
                ),
            ),
            TomlLineGroup::KeyValue(
                Coword(
                    "autobenches",
                ),
                Some(
                    5,
                ),
            ),
            TomlLineGroup::Comment,
            TomlLineGroup::Comment,
            TomlLineGroup::KeyValue(
                Coword(
                    "edition",
                ),
                Some(
                    6,
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
                    7,
                ),
            ),
            TomlLineGroup::KeyValue(
                Coword(
                    "bench",
                ),
                Some(
                    8,
                ),
            ),
        ],
        table: TomlTable {
            data: {
                Coword(
                    "lib",
                ): Section(
                    1,
                ),
                Coword(
                    "package",
                ): Section(
                    0,
                ),
            },
        },
    },
)
```