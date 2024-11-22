use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LpCsvFile {
    Rows(Vec<LpCsvRow>),
}

impl<'a> LpCsvParser<'a> {
    pub(crate) fn parse_file(mut self) -> LpCsvResult<LpCsvFile> {
        let mut rows = Vec::new();
        while let Some(row) = self.parse_row().into_result_option()? {
            rows.push(row);
        }
        Ok(LpCsvFile::Rows(rows))
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
            Rows(
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
            )
        "#]],
    );
    t(
        r#"mathbb,1,2
mathcal,1,2"#,
        expect![[r#"
            Rows(
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
            )
        "#]],
    );
}
