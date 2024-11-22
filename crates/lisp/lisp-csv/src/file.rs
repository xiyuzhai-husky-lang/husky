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
                        Literal(
                            Integer(
                                1,
                            ),
                        ),
                        Literal(
                            Integer(
                                2,
                            ),
                        ),
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
                            Ident(
                                "mathbb",
                            ),
                            Literal(
                                Integer(
                                    1,
                                ),
                            ),
                            Literal(
                                Integer(
                                    2,
                                ),
                            ),
                        ],
                    ),
                    SeparatedExprs(
                        [
                            Ident(
                                "mathcal",
                            ),
                            Literal(
                                Integer(
                                    1,
                                ),
                            ),
                            Literal(
                                Integer(
                                    2,
                                ),
                            ),
                        ],
                    ),
                ],
            )
        "#]],
    );
}
