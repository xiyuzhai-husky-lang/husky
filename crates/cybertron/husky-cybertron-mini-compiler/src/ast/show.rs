use super::*;

pub fn show_asts(tokens: Seq<Token>, asts: Seq<Option<Ast>>) -> Vec<(String, String)> {
    let tokens = tokens.data();
    let asts = asts.data();
    let len = tokens.len();
    debug_assert_eq!(len, asts.len());
    let mut outs: Vec<(String, String)> = (0..len)
        .into_iter()
        .map(|i| (tokens[i].repr(), "".into()))
        .collect();
    for i in 0..len {
        calc_ast_repr(tokens, asts, idx!(i), &mut outs)
    }
    outs
}

fn calc_ast_repr(
    tokens: &[Token],
    asts: &[Option<Ast>],
    index: Idx,
    outs: &mut Vec<(String, String)>,
) {
    let i = index.index();
    let Some(ast) = asts[i] else { return };
    if outs[i].1.len() > 0 {
        return;
    }
    let repr = match ast.data {
        AstData::Literal(lit) => lit.repr(),
        AstData::Ident(ident) => ident.repr().to_string(),
        AstData::Prefix { opr, opd } => {
            calc_ast_repr(tokens, asts, opd, outs);
            format!("{}{}", opr.repr_short(), outs[opd.index()].1)
        }
        AstData::Binary { lopd, opr, ropd } => {
            calc_ast_repr(tokens, asts, lopd, outs);
            calc_ast_repr(tokens, asts, ropd, outs);
            format!(
                "{} {} {}",
                outs[lopd.index()].1,
                opr.repr_short(),
                outs[ropd.index()].1
            )
        }
        AstData::Suffix { opd, opr } => {
            calc_ast_repr(tokens, asts, opd, outs);
            format!("{}{}", outs[opd.index()].1, opr.repr())
        }
        AstData::Delimited {
            left_delimiter_idx,
            left_delimiter,
            right_delimiter,
        } => {
            let mut result = String::new();
            result += left_delimiter.repr();
            for (jj, (j, ast)) in asts
                .iter()
                .copied()
                .enumerate()
                .filter_map(|(i, ast)| Some((i, ast?)))
                .enumerate()
            {
                calc_ast_repr(tokens, asts, idx!(j), outs);
                result += &outs[j].1;
            }
            result += right_delimiter.repr();
            result
        }
        AstData::SeparatedItem { content, separator } => {
            let mut result = String::new();
            if let Some(content) = content {
                calc_ast_repr(tokens, asts, content, outs);
                result += &outs[content.index()].1;
            }
            result += separator.repr();
            result
        }
        AstData::CallOrIndex { caller, arguments } => {
            calc_ast_repr(tokens, asts, caller, outs);
            calc_ast_repr(tokens, asts, arguments, outs);
            format!("{}{}", outs[caller.index()].1, outs[arguments.index()].1)
        }
        AstData::Defn {
            keyword,
            name,
            data,
        } => match data {
            DefnData::Type { content } => {
                calc_ast_repr(tokens, asts, content, outs);
                format!(
                    "{} {} {}",
                    keyword.repr(),
                    name.repr(),
                    outs[content.index()].1
                )
            }
            DefnData::Func { head, body } => {
                calc_ast_repr(tokens, asts, head, outs);
                calc_ast_repr(tokens, asts, body, outs);
                format!(
                    "{} {} {} {}",
                    keyword.repr(),
                    name.repr(),
                    outs[head.index()].1,
                    outs[body.index()].1
                )
            }
        },
        AstData::LetInit {
            pattern,
            initial_value,
        } => {
            calc_ast_repr(tokens, asts, pattern, outs);
            calc_ast_repr(tokens, asts, initial_value, outs);
            format!(
                "let {} = {}",
                outs[pattern.index()].1,
                outs[initial_value.index()].1
            )
        }
    };
    outs[i].1 = repr
}

#[test]
fn show_asts_works() {
    expect![[r#"
        [
            (
                "hello",
                "hello",
            ),
        ]
    "#]]
    .assert_debug_eq(&show_asts(
        seq![Token::Ident(Ident::new("hello"))],
        seq![Some(Ast {
            parent: None,
            data: AstData::Ident(Ident::new("hello"))
        })],
    ));
    expect![[r#"
        [
            (
                "+(plus)",
                "+hello",
            ),
            (
                "hello",
                "hello",
            ),
        ]
    "#]]
    .assert_debug_eq(&show_asts(
        seq![Token::Opr(Opr::PLUS), Token::Ident(Ident::new("hello"))],
        seq![
            Some(Ast {
                parent: None,
                data: AstData::Prefix {
                    opr: PrefixOpr::Plus,
                    opd: idx!(1)
                }
            }),
            Some(Ast {
                parent: Some(idx!(0)),
                data: AstData::Ident(Ident::new("hello"))
            })
        ],
    ));
    expect![[r#"
        [
            (
                "1",
                "1",
            ),
            (
                "+(add)",
                "1 + 1",
            ),
            (
                "1",
                "1",
            ),
        ]
    "#]]
    .assert_debug_eq(&show_asts(
        seq![
            Token::Literal(Literal::Int(1)),
            Token::Opr(Opr::ADD),
            Token::Literal(Literal::Int(1))
        ],
        seq![
            Some(Ast {
                parent: Some(idx!(1)),
                data: AstData::Literal(Literal::Int(1))
            }),
            Some(Ast {
                parent: None,
                data: AstData::Binary {
                    lopd: idx!(0),
                    opr: BinaryOpr::Add,
                    ropd: idx!(2)
                }
            }),
            Some(Ast {
                parent: Some(idx!(1)),
                data: AstData::Literal(Literal::Int(1))
            })
        ],
    ));
}
