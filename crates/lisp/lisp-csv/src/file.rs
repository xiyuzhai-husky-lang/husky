use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LpCsvFile {
    input: String,
    data: LpCsvFileData,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LpCsvFileData {
    Rows(Vec<LpCsvRow>),
}

impl LpCsvFile {
    pub fn input(&self) -> &str {
        &self.input
    }

    pub fn data(&self) -> &LpCsvFileData {
        &self.data
    }
}

impl<'a> LpCsvParser<'a> {
    pub(crate) fn parse_file(mut self) -> LpCsvResult<LpCsvFile> {
        let mut rows = Vec::new();
        while let Some(row) = self.parse_row().into_result_option()? {
            rows.push(row);
        }
        Ok(LpCsvFile {
            input: self.input.to_string(),
            data: LpCsvFileData::Rows(rows),
        })
    }
}

#[test]
fn parse_lp_csv_file_works() {
    fn t(input: &str, expected: Expect) {
        let mut parser = LpCsvParser::new(input);
        let file = parser.parse_file().unwrap();
        expected.assert_debug_eq(&file);
    }
    t(
        "1,2",
        expect![[r#"
            LpCsvFile {
                input: "1,2",
                data: Rows(
                    [
                        SeparatedExprs(
                            [
                                LpCsvExpr {
                                    data: Literal(
                                        Integer(
                                            1,
                                        ),
                                    ),
                                    offset_range: 0..1,
                                    position_range: [1:1, 1:2),
                                },
                                LpCsvExpr {
                                    data: Literal(
                                        Integer(
                                            2,
                                        ),
                                    ),
                                    offset_range: 2..3,
                                    position_range: [1:3, 1:4),
                                },
                            ],
                        ),
                    ],
                ),
            }
        "#]],
    );
    t(
        r#"mathbb,1,2
mathcal,1,2"#,
        expect![[r#"
            LpCsvFile {
                input: "mathbb,1,2\nmathcal,1,2",
                data: Rows(
                    [
                        SeparatedExprs(
                            [
                                LpCsvExpr {
                                    data: Ident(
                                        "mathbb",
                                    ),
                                    offset_range: 0..6,
                                    position_range: [1:1, 1:7),
                                },
                                LpCsvExpr {
                                    data: Literal(
                                        Integer(
                                            1,
                                        ),
                                    ),
                                    offset_range: 7..8,
                                    position_range: [1:8, 1:9),
                                },
                                LpCsvExpr {
                                    data: Literal(
                                        Integer(
                                            2,
                                        ),
                                    ),
                                    offset_range: 9..10,
                                    position_range: [1:10, 1:11),
                                },
                            ],
                        ),
                        SeparatedExprs(
                            [
                                LpCsvExpr {
                                    data: Ident(
                                        "mathcal",
                                    ),
                                    offset_range: 11..18,
                                    position_range: [2:1, 2:8),
                                },
                                LpCsvExpr {
                                    data: Literal(
                                        Integer(
                                            1,
                                        ),
                                    ),
                                    offset_range: 19..20,
                                    position_range: [2:9, 2:10),
                                },
                                LpCsvExpr {
                                    data: Literal(
                                        Integer(
                                            2,
                                        ),
                                    ),
                                    offset_range: 21..22,
                                    position_range: [2:11, 2:12),
                                },
                            ],
                        ),
                    ],
                ),
            }
        "#]],
    );
    t(
        r#"
//! format:
//! <instance-ident> = <instantiation>, <signature-data-variant-path> <arg0> <arg1> ...
//! <instantiation> = <item-path> <arg0> <arg1> ...
# instances of lisp-csv

## prefix operators
### pos
int_pos = ring_pos int, base_prefix_opr int int
rat_pos = ring_pos rat, base_prefix_opr rat rat
real_pos = ring_pos real, base_prefix_opr real real
complex_pos = ring_pos complex, base_prefix_opr complex complex
"#,
        expect![[r#"
            LpCsvFile {
                input: "\n//! format:\n//! <instance-ident> = <instantiation>, <signature-data-variant-path> <arg0> <arg1> ...\n//! <instantiation> = <item-path> <arg0> <arg1> ...\n# instances of lisp-csv\n\n## prefix operators\n### pos\nint_pos = ring_pos int, base_prefix_opr int int\nrat_pos = ring_pos rat, base_prefix_opr rat rat\nreal_pos = ring_pos real, base_prefix_opr real real\ncomplex_pos = ring_pos complex, base_prefix_opr complex complex\n",
                data: Rows(
                    [
                        SeparatedExprs(
                            [
                                LpCsvExpr {
                                    data: Ident(
                                        "int_pos",
                                    ),
                                    offset_range: 206..213,
                                    position_range: [9:1, 9:8),
                                },
                                LpCsvExpr {
                                    data: Application(
                                        [
                                            LpCsvExpr {
                                                data: Ident(
                                                    "ring_pos",
                                                ),
                                                offset_range: 216..224,
                                                position_range: [9:11, 9:19),
                                            },
                                            LpCsvExpr {
                                                data: Ident(
                                                    "int",
                                                ),
                                                offset_range: 225..228,
                                                position_range: [9:20, 9:23),
                                            },
                                        ],
                                    ),
                                    offset_range: 216..228,
                                    position_range: [9:11, 9:23),
                                },
                                LpCsvExpr {
                                    data: Application(
                                        [
                                            LpCsvExpr {
                                                data: Ident(
                                                    "base_prefix_opr",
                                                ),
                                                offset_range: 230..245,
                                                position_range: [9:25, 9:40),
                                            },
                                            LpCsvExpr {
                                                data: Ident(
                                                    "int",
                                                ),
                                                offset_range: 246..249,
                                                position_range: [9:41, 9:44),
                                            },
                                            LpCsvExpr {
                                                data: Ident(
                                                    "int",
                                                ),
                                                offset_range: 250..253,
                                                position_range: [9:45, 9:48),
                                            },
                                        ],
                                    ),
                                    offset_range: 230..253,
                                    position_range: [9:25, 9:48),
                                },
                            ],
                        ),
                        SeparatedExprs(
                            [
                                LpCsvExpr {
                                    data: Ident(
                                        "rat_pos",
                                    ),
                                    offset_range: 254..261,
                                    position_range: [10:1, 10:8),
                                },
                                LpCsvExpr {
                                    data: Application(
                                        [
                                            LpCsvExpr {
                                                data: Ident(
                                                    "ring_pos",
                                                ),
                                                offset_range: 264..272,
                                                position_range: [10:11, 10:19),
                                            },
                                            LpCsvExpr {
                                                data: Ident(
                                                    "rat",
                                                ),
                                                offset_range: 273..276,
                                                position_range: [10:20, 10:23),
                                            },
                                        ],
                                    ),
                                    offset_range: 264..276,
                                    position_range: [10:11, 10:23),
                                },
                                LpCsvExpr {
                                    data: Application(
                                        [
                                            LpCsvExpr {
                                                data: Ident(
                                                    "base_prefix_opr",
                                                ),
                                                offset_range: 278..293,
                                                position_range: [10:25, 10:40),
                                            },
                                            LpCsvExpr {
                                                data: Ident(
                                                    "rat",
                                                ),
                                                offset_range: 294..297,
                                                position_range: [10:41, 10:44),
                                            },
                                            LpCsvExpr {
                                                data: Ident(
                                                    "rat",
                                                ),
                                                offset_range: 298..301,
                                                position_range: [10:45, 10:48),
                                            },
                                        ],
                                    ),
                                    offset_range: 278..301,
                                    position_range: [10:25, 10:48),
                                },
                            ],
                        ),
                        SeparatedExprs(
                            [
                                LpCsvExpr {
                                    data: Ident(
                                        "real_pos",
                                    ),
                                    offset_range: 302..310,
                                    position_range: [11:1, 11:9),
                                },
                                LpCsvExpr {
                                    data: Application(
                                        [
                                            LpCsvExpr {
                                                data: Ident(
                                                    "ring_pos",
                                                ),
                                                offset_range: 313..321,
                                                position_range: [11:12, 11:20),
                                            },
                                            LpCsvExpr {
                                                data: Ident(
                                                    "real",
                                                ),
                                                offset_range: 322..326,
                                                position_range: [11:21, 11:25),
                                            },
                                        ],
                                    ),
                                    offset_range: 313..326,
                                    position_range: [11:12, 11:25),
                                },
                                LpCsvExpr {
                                    data: Application(
                                        [
                                            LpCsvExpr {
                                                data: Ident(
                                                    "base_prefix_opr",
                                                ),
                                                offset_range: 328..343,
                                                position_range: [11:27, 11:42),
                                            },
                                            LpCsvExpr {
                                                data: Ident(
                                                    "real",
                                                ),
                                                offset_range: 344..348,
                                                position_range: [11:43, 11:47),
                                            },
                                            LpCsvExpr {
                                                data: Ident(
                                                    "real",
                                                ),
                                                offset_range: 349..353,
                                                position_range: [11:48, 11:52),
                                            },
                                        ],
                                    ),
                                    offset_range: 328..353,
                                    position_range: [11:27, 11:52),
                                },
                            ],
                        ),
                        SeparatedExprs(
                            [
                                LpCsvExpr {
                                    data: Ident(
                                        "complex_pos",
                                    ),
                                    offset_range: 354..365,
                                    position_range: [12:1, 12:12),
                                },
                                LpCsvExpr {
                                    data: Application(
                                        [
                                            LpCsvExpr {
                                                data: Ident(
                                                    "ring_pos",
                                                ),
                                                offset_range: 368..376,
                                                position_range: [12:15, 12:23),
                                            },
                                            LpCsvExpr {
                                                data: Ident(
                                                    "complex",
                                                ),
                                                offset_range: 377..384,
                                                position_range: [12:24, 12:31),
                                            },
                                        ],
                                    ),
                                    offset_range: 368..384,
                                    position_range: [12:15, 12:31),
                                },
                                LpCsvExpr {
                                    data: Application(
                                        [
                                            LpCsvExpr {
                                                data: Ident(
                                                    "base_prefix_opr",
                                                ),
                                                offset_range: 386..401,
                                                position_range: [12:33, 12:48),
                                            },
                                            LpCsvExpr {
                                                data: Ident(
                                                    "complex",
                                                ),
                                                offset_range: 402..409,
                                                position_range: [12:49, 12:56),
                                            },
                                            LpCsvExpr {
                                                data: Ident(
                                                    "complex",
                                                ),
                                                offset_range: 410..417,
                                                position_range: [12:57, 12:64),
                                            },
                                        ],
                                    ),
                                    offset_range: 386..417,
                                    position_range: [12:33, 12:64),
                                },
                            ],
                        ),
                    ],
                ),
            }
        "#]],
    );
}
