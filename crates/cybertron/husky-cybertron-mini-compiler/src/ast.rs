use crate::{
    token::{ident::Ident, keyword::Keyword, literal::Literal, opr::Opr, Token},
    *,
};
use husky_cybertron::seq::{idx::Idx, Seq};
use token::tokenize;

#[enum_class::from_variants]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ast {
    Literal(Literal),
    Ident(Ident),
    /// # exprs
    Binary {
        lopd: Idx,
        ropd: Idx,
    },
    /// # stmts
    LetInit,
}

pub enum PreAst {
    Keyword(Keyword),
    Opr(Opr),
    Ast(Ast),
}

impl std::fmt::Debug for PreAst {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Keyword(slf) => slf.fmt(f),
            Self::Opr(slf) => slf.fmt(f),
            Self::Ast(slf) => slf.fmt(f),
        }
    }
}

impl From<Token> for PreAst {
    fn from(tok: Token) -> Self {
        match tok {
            Token::Keyword(kw) => PreAst::Keyword(kw),
            Token::Ident(ident) => PreAst::Ast(Ast::Ident(ident)),
            Token::Opr(opr) => PreAst::Opr(opr),
        }
    }
}

pub fn calc_pre_ast_initial_seq(toks: Seq<Token>) -> Seq<PreAst> {
    toks.map(Into::into)
}

#[test]
fn calc_pre_ast_initial_seq_works() {
    fn t(input: &str, expect: Expect) {
        expect.assert_debug_eq(&calc_pre_ast_initial_seq(tokenize(input)))
    }
    t(
        "hello",
        expect![[r#"
            [Ident(i`hello`)]
        "#]],
    );
    t(
        "let hello = world + humans",
        expect![[r#"
            [
                Let,
                Ident(
                    i`hello`,
                ),
                Binary(
                    Assign,
                ),
                Ident(
                    i`world`,
                ),
                Binary(
                    Add,
                ),
                Ident(
                    i`humans`,
                ),
            ]
        "#]],
    );
}
