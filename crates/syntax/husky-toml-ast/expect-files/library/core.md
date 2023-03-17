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
                            (
                                2,
                                Word(
                                    "version",
                                ),
                                Some(
                                    1,
                                ),
                            ),
                            (
                                3,
                                Word(
                                    "license",
                                ),
                                Some(
                                    2,
                                ),
                            ),
                            (
                                4,
                                Word(
                                    "repository",
                                ),
                                Some(
                                    3,
                                ),
                            ),
                            (
                                5,
                                Word(
                                    "description",
                                ),
                                Some(
                                    4,
                                ),
                            ),
                            (
                                6,
                                Word(
                                    "autotests",
                                ),
                                Some(
                                    5,
                                ),
                            ),
                            (
                                7,
                                Word(
                                    "autobenches",
                                ),
                                Some(
                                    6,
                                ),
                            ),
                            (
                                10,
                                Word(
                                    "edition",
                                ),
                                Some(
                                    7,
                                ),
                            ),
                        ],
                    },
                    TomlSectionAst {
                        title: TomlSectionTitle {
                            words: [
                                Word(
                                    "lib",
                                ),
                            ],
                        },
                        kind: TomlSectionKind::Normal,
                        key_value_pairs: [
                            (
                                12,
                                Word(
                                    "test",
                                ),
                                Some(
                                    8,
                                ),
                            ),
                            (
                                13,
                                Word(
                                    "bench",
                                ),
                                Some(
                                    9,
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
            TomlGroup::KeyValue(
                Word(
                    "version",
                ),
                Some(
                    1,
                ),
            ),
            TomlGroup::KeyValue(
                Word(
                    "license",
                ),
                Some(
                    2,
                ),
            ),
            TomlGroup::KeyValue(
                Word(
                    "repository",
                ),
                Some(
                    3,
                ),
            ),
            TomlGroup::KeyValue(
                Word(
                    "description",
                ),
                Some(
                    4,
                ),
            ),
            TomlGroup::KeyValue(
                Word(
                    "autotests",
                ),
                Some(
                    5,
                ),
            ),
            TomlGroup::KeyValue(
                Word(
                    "autobenches",
                ),
                Some(
                    6,
                ),
            ),
            TomlGroup::Comment,
            TomlGroup::Comment,
            TomlGroup::KeyValue(
                Word(
                    "edition",
                ),
                Some(
                    7,
                ),
            ),
            TomlGroup::SectionTitle {
                title: [
                    Word(
                        "lib",
                    ),
                ],
                kind: TomlSectionKind::Normal,
            },
            TomlGroup::KeyValue(
                Word(
                    "test",
                ),
                Some(
                    8,
                ),
            ),
            TomlGroup::KeyValue(
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
                        value: 1,
                    },
                ): Section(
                    0,
                ),
                Word(
                    Id {
                        value: 11,
                    },
                ): Section(
                    1,
                ),
            },
        },
    },
)