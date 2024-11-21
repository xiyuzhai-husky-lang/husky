use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LpCsvExpr {
    Literal(LpCsvLiteral),
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
            _ => Some(LpCsvExpr::List(list)),
        }
    }

    fn parse_expr_aux(&mut self) -> Option<LpCsvExpr> {
        self.ignore_whitespaces_and_tabs_and_blank_lines_and_comments();
        match self.chars.peek()? {
            '"' => {
                self.chars.eat_char();
                let mut s = String::new();
                loop {
                    match self.chars.next() {
                        Some('"') => break,
                        Some('\\') => match self.chars.next() {
                            Some('n') => s.push('\n'),
                            Some('t') => s.push('\t'),
                            Some('\\') => s.push('\\'),
                            Some('"') => s.push('"'),
                            Some(c) => todo!("c: `{c:?}`"),
                            None => todo!(),
                        },
                        Some(c) => s.push(c),
                        None => todo!(),
                    }
                }
                Some(LpCsvExpr::Literal(LpCsvLiteral::String(s)))
            }
            c if c.is_ascii_digit() => {
                let mut dot_count = 0;
                let s = self.chars.next_str_slice_while(|c| {
                    if c == '.' {
                        dot_count += 1;
                    }
                    (c.is_ascii_digit() || c == '.') && dot_count < 2
                });
                let literal = match dot_count {
                    0 => {
                        let i = match s.parse() {
                            Ok(i) => i,
                            Err(e) => todo!("{}", e),
                        };
                        LpCsvLiteral::Integer(i)
                    }
                    1 => {
                        let f = match s.parse::<f64>() {
                            Ok(f) => f,
                            Err(e) => todo!("{}", e),
                        };
                        let f = OrderedFloat(f);
                        LpCsvLiteral::Float(f)
                    }
                    _ => unreachable!(),
                };
                Some(LpCsvExpr::Literal(literal))
            }
            c if c.is_ascii_alphabetic() => {
                let ident = self
                    .chars
                    .next_str_slice_while(|c| c.is_ascii_alphanumeric() || c == '_')
                    .to_string();
                Some(LpCsvExpr::Ident(ident))
            }
            '(' => {
                self.chars.eat_char();
                let expr = self.parse_expr();
                let Some(c) = self.chars.next() else { todo!() };
                if c != ')' {
                    todo!()
                }
                match expr {
                    Some(expr) => Some(LpCsvExpr::Parenthesized(Box::new(expr))),
                    None => todo!(),
                }
            }
            ',' | ';' | ')' => None,
            c => todo!("c: `{c:?}"),
        }
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
                    List(
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
                    List(
                        [
                            Ident(
                                "add",
                            ),
                            Parenthesized(
                                List(
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
}
