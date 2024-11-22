use token::LpCsvToken;

use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LpCsvExpr {
    Literal(LpCsvLiteral),
    Application(Vec<LpCsvExpr>),
    List(Vec<LpCsvExpr>),
    Ident(String),
    Parenthesized(Box<LpCsvExpr>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LpCsvLiteral {
    String(String),
    Integer(i64),
    Float(OrderedFloat<f64>),
}

impl<'a> LpCsvParser<'a> {
    pub(crate) fn parse_expr(&mut self) -> Option<LpCsvExpr> {
        let mut list: Vec<LpCsvExpr> = vec![];
        while let Some(expr) = self.parse_expr_aux() {
            list.push(expr);
        }
        match list.len() {
            0 => None,
            1 => list.pop(),
            _ => Some(LpCsvExpr::Application(list)),
        }
    }

    fn parse_expr_aux(&mut self) -> Option<LpCsvExpr> {
        self.ignore_whitespaces_and_tabs_and_comments();
        match self.peek_token()? {
            LpCsvToken::Literal(literal) => {
                self.eat_token();
                Some(LpCsvExpr::Literal(literal))
            }
            // TODO: handle connectors followed by ident
            LpCsvToken::Ident(ident) => {
                self.eat_token();
                Some(LpCsvExpr::Ident(ident))
            }
            LpCsvToken::Connector(connector) => None,
            LpCsvToken::Separator(separator) => None,
            LpCsvToken::LeftParen => {
                self.eat_token();
                Some(LpCsvExpr::Parenthesized(self.parse_parenthesized_expr()))
            }
            LpCsvToken::RightParen => None,
            LpCsvToken::LeftBracket => {
                self.eat_token();
                Some(LpCsvExpr::List(self.parse_list_expr()))
            }
            LpCsvToken::RightBracket => None,
        }
    }

    fn parse_parenthesized_expr(&mut self) -> Box<LpCsvExpr> {
        let Some(expr) = self.parse_expr() else {
            todo!()
        };
        let Some(token) = self.next_token() else {
            todo!()
        };
        if token != LpCsvToken::RightParen {
            todo!()
        }
        Box::new(expr)
    }

    fn parse_list_expr(&mut self) -> Vec<LpCsvExpr> {
        let mut list: Vec<LpCsvExpr> = vec![];
        while let Some(expr) = self.parse_expr() {
            list.push(expr);
            self.ignore_whitespaces_and_tabs_and_comments();
            match self.chars.next() {
                Some(']') => break,
                Some(c) if self.is_list_item_separator(c) => (),
                _ => todo!(),
            }
        }
        list
    }
}

#[test]
fn parse_lp_csv_expr_works() {
    fn t(input: &str, expect: Expect) {
        let mut parser = LpCsvParser::new(input);
        let actual = parser.parse_expr();
        expect.assert_debug_eq(&actual);
    }
    t(
        "",
        expect!([r#"
        None
    "#]),
    );
    t(
        "1",
        expect!([r#"
            Some(
                Literal(
                    Integer(
                        1,
                    ),
                ),
            )
        "#]),
    );
    t(
        "1.2",
        expect!([r#"
            Some(
                Literal(
                    Float(
                        1.2,
                    ),
                ),
            )
        "#]),
    );
    t(
        r#""1%&safdh\\command""#,
        expect!([r#"
            Some(
                Literal(
                    String(
                        "1%&safdh\\command",
                    ),
                ),
            )
        "#]),
    );
    t(
        "x",
        expect!([r#"
            Some(
                Ident(
                    "x",
                ),
            )
        "#]),
    );
    t(
        "x1",
        expect!([r#"
            Some(
                Ident(
                    "x1",
                ),
            )
        "#]),
    );
    t(
        "hello_world",
        expect!([r#"
            Some(
                Ident(
                    "hello_world",
                ),
            )
        "#]),
    );
    t(
        "hello_world,",
        expect!([r#"
            Some(
                Ident(
                    "hello_world",
                ),
            )
        "#]),
    );
    t(
        "(define x)",
        expect!([r#"
            Some(
                Parenthesized(
                    Application(
                        [
                            Ident(
                                "define",
                            ),
                            Ident(
                                "x",
                            ),
                        ],
                    ),
                ),
            )
        "#]),
    );
    t(
        "(add (define x) 1)",
        expect!([r#"
            Some(
                Parenthesized(
                    Application(
                        [
                            Ident(
                                "add",
                            ),
                            Parenthesized(
                                Application(
                                    [
                                        Ident(
                                            "define",
                                        ),
                                        Ident(
                                            "x",
                                        ),
                                    ],
                                ),
                            ),
                            Literal(
                                Integer(
                                    1,
                                ),
                            ),
                        ],
                    ),
                ),
            )
        "#]),
    );
    t(
        r#"1,2
3,4"#,
        expect![[r#"
            Some(
                Literal(
                    Integer(
                        1,
                    ),
                ),
            )
        "#]],
    );
    t(
        r#"x
3"#,
        expect![[r#"
            Some(
                Ident(
                    "x",
                ),
            )
        "#]],
    );
    t(
        "[1,2,3]",
        expect![[r#"
        Some(
            List(
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
                    Literal(
                        Integer(
                            3,
                        ),
                    ),
                ],
            ),
        )
    "#]],
    );
    t(
        "f [1,2,3]",
        expect![[r#"
            Some(
                Application(
                    [
                        Ident(
                            "f",
                        ),
                        List(
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
                                Literal(
                                    Integer(
                                        3,
                                    ),
                                ),
                            ],
                        ),
                    ],
                ),
            )
        "#]],
    );
}
