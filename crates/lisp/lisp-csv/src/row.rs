use crate::*;
use maybe_result::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LpCsvRow {
    Expr(LpCsvExpr),
    SeparatedExprs(Vec<LpCsvExpr>),
}

impl<'a> LpCsvParser<'a> {
    pub(crate) fn parse_row(&mut self) -> LpCsvMaybeResult<LpCsvRow> {
        self.ignore_whitespaces_and_tabs_and_blank_lines_and_comments();
        let expr = self.parse_expr()?;
        self.ignore_whitespaces_and_tabs_and_comments();
        match self.chars.peek() {
            Some(',') => {
                let mut exprs: Vec<LpCsvExpr> = vec![expr];
                loop {
                    self.chars.eat_char();
                    let Some(expr) = self.parse_expr() else {
                        todo!()
                    };
                    exprs.push(expr);
                    self.ignore_whitespaces_and_tabs_and_comments();
                    match self.chars.peek() {
                        Some(',') => (),
                        Some('\n') | None => break,
                        Some(_) => todo!(),
                    }
                }
                JustOk(LpCsvRow::SeparatedExprs(exprs))
            }
            Some('\n') | None => JustOk(LpCsvRow::Expr(expr)),
            Some(_) => todo!(),
        }
    }
}

#[test]
fn parse_lp_csv_row_works() {
    fn t(input: &str, expect: Expect) {
        let mut parser = LpCsvParser::new(input);
        let row = parser.parse_row();
        expect.assert_debug_eq(&row);
    }
    t(
        "",
        expect![[r#"
        Nothing
    "#]],
    );
    t(
        "1",
        expect![[r#"
        JustOk(
            Expr(
                Literal(
                    Integer(
                        1,
                    ),
                ),
            ),
        )
    "#]],
    );
    t(
        "1,2",
        expect![[r#"
            JustOk(
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
            )
        "#]],
    );
    t(
        "mathbb,1,2",
        expect![[r#"
            JustOk(
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
            )
        "#]],
    );
    t(
        r#"1,2
3,4"#,
        expect![[r#"
            JustOk(
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
            )
        "#]],
    );
}
