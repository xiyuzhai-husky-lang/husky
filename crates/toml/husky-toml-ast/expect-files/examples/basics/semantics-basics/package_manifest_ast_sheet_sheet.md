Ok(
    TomlAstSheet {
        expr_arena: Arena {
            data: [
                TomlExpr::String(
                    "semantics-basics",
                ),
                TomlExpr::String(
                    "0.1.0",
                ),
                TomlExpr::String(
                    "example mnist classifier",
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
                                    1,
                                ),
                            },
                            TomlSectionEntry {
                                line_group_idx: TomlLineGroupIdx(
                                    2,
                                ),
                                key: Coword(
                                    Id {
                                        value: 4,
                                    },
                                ),
                                value: Some(
                                    2,
                                ),
                            },
                            TomlSectionEntry {
                                line_group_idx: TomlLineGroupIdx(
                                    3,
                                ),
                                key: Coword(
                                    Id {
                                        value: 5,
                                    },
                                ),
                                value: Some(
                                    3,
                                ),
                            },
                            TomlSectionEntry {
                                line_group_idx: TomlLineGroupIdx(
                                    4,
                                ),
                                key: Coword(
                                    Id {
                                        value: 6,
                                    },
                                ),
                                value: Some(
                                    4,
                                ),
                            },
                        ],
                    },
                    TomlSection {
                        title: TomlSectionTitle(
                            [
                                Coword(
                                    Id {
                                        value: 7,
                                    },
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
            TomlLineGroup::KeyValue(
                Word(
                    "version",
                ),
                Some(
                    2,
                ),
            ),
            TomlLineGroup::KeyValue(
                Word(
                    "description",
                ),
                Some(
                    3,
                ),
            ),
            TomlLineGroup::KeyValue(
                Word(
                    "license",
                ),
                Some(
                    4,
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
        ],
        table: TomlTable {
            data: {
                Coword(
                    Id {
                        value: 2,
                    },
                ): Section(
                    1,
                ),
                Coword(
                    Id {
                        value: 7,
                    },
                ): Section(
                    2,
                ),
            },
        },
    },
)