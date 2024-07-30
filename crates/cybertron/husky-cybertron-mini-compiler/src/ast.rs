mod alloc;
mod reduce;

use self::{alloc::*, reduce::*};
use crate::{
    token::{ident::Ident, keyword::Keyword, literal::Literal, opr::Opr, Token},
    *,
};
use husky_cybertron::seq::idx::Option2;
use husky_cybertron::{
    prelude::*,
    seq::{idx::Idx, Seq},
};
use token::{
    opr::{BinaryOpr, PrefixOpr, SuffixOpr},
    tokenize,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ast {
    pub parent: Option<Idx>,
    pub data: AstData,
}

#[enum_class::from_variants]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AstData {
    Literal(Literal),
    Ident(Ident),
    /// # exprs
    Prefix {
        opr: PrefixOpr,
        opd: Idx,
    },
    Binary {
        lopd: Idx,
        opr: BinaryOpr,
        ropd: Idx,
    },
    Suffix {
        opd: Idx,
        opr: SuffixOpr,
    },
    /// # stmts
    LetInit,
}

impl Into<Option<Ast>> for Token {
    fn into(self) -> Option<Ast> {
        let data = match self {
            Token::Literal(lit) => Some(AstData::Literal(lit)),
            Token::Keyword(_) => None,
            Token::Ident(ident) => Some(AstData::Ident(ident)),
            Token::Opr(_) => None,
        }?;
        Some(Ast { parent: None, data })
    }
}

#[enum_class::from_variants]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AstError {
    Opr,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PreAst {
    Keyword(Keyword),
    Opr(Opr),
    Ast(AstData),
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
            Token::Ident(ident) => PreAst::Ast(AstData::Ident(ident)),
            Token::Opr(opr) => PreAst::Opr(opr),
            Token::Literal(lit) => PreAst::Ast(AstData::Literal(lit)),
        }
    }
}

pub fn calc_pre_ast_initial_seq(toks: Seq<Token>) -> Seq<Option<PreAst>> {
    toks.map(|tok| Some(tok.into()))
}

#[test]
fn calc_pre_ast_initial_seq_works() {
    fn t(input: &str, expect: Expect) {
        expect.assert_debug_eq(&calc_pre_ast_initial_seq(tokenize(input)))
    }
    t(
        "hello",
        expect![[r#"
            [Some(Ident(`hello`))]
        "#]],
    );
    t(
        "let hello = world + humans",
        expect![[r#"
            [
                Some(
                    `let`,
                ),
                Some(
                    Ident(
                        `hello`,
                    ),
                ),
                Some(
                    `=`,
                ),
                Some(
                    Ident(
                        `world`,
                    ),
                ),
                Some(
                    `+(add)`,
                ),
                Some(
                    Ident(
                        `humans`,
                    ),
                ),
            ]
        "#]],
    );
}
