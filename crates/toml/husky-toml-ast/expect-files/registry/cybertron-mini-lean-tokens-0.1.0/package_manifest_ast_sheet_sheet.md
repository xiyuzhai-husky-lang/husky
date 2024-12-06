```rust
Ok(
    TomlAstSheet {
        expr_arena: Arena {
            data: [
                TomlExpr::String(
                    "cybertron-mini-lean-tokens",
                ),
                TomlExpr::String(
                    "tokens of example mini-lean-compiler in cybertron",
                ),
                TomlExpr::String(
                    "MIT OR Apache-2.0",
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
                                    "description",
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
                                    "license",
                                ),
                                value: Some(
                                    2,
                                ),
                            },
                        ],
                    },
                    TomlSection {
                        title: TomlSectionTitle(
                            [
                                Coword(
                                    "dependencies",
                                ),
                            ],
                        ),
                        kind: TomlSectionKind::Normal,
                        entries: [],
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
                    "description",
                ),
                Some(
                    1,
                ),
            ),
            TomlLineGroup::KeyValue(
                Coword(
                    "license",
                ),
                Some(
                    2,
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
        ],
        table: TomlTable {
            data: {
                Coword(
                    "dependencies",
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