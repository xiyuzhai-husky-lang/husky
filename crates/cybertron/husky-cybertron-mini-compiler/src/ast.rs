mod alloc;
mod reduce;
pub mod show;

use std::fmt::Pointer;

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
    delimiter::{LeftDelimiter, RightDelimiter},
    opr::{BinaryOpr, PrefixOpr, SuffixOpr},
    separator::Separator,
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
    Delimited {
        left_delimiter_idx: Idx,
        left_delimiter: LeftDelimiter,
        right_delimiter: RightDelimiter,
    },
    SeparatedItem {
        content: Option<Idx>,
        separator: Separator,
    },
    /// things like `f(...)` or `a[...]`
    Call {
        caller: Idx,
        delimited_arguments: Idx,
    },
    /// # stmts
    LetInit {
        expr: Idx,
        pattern: Idx,
        initial_value: Option<Idx>,
    },
    If {
        condition: Idx,
        body: Idx,
    },
    Else {
        if_stmt: Idx,
        body: Idx,
    },
    /// # defn
    Defn {
        keyword: Keyword,
        name: Ident,
        data: DefnData,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DefnData {
    Type { content: Idx },
    Func { head: Idx, body: Idx },
}

impl Into<Option<Ast>> for Token {
    fn into(self) -> Option<Ast> {
        let data = match self {
            Token::Literal(lit) => Some(AstData::Literal(lit)),
            Token::Keyword(_) => None,
            Token::Ident(ident) => Some(AstData::Ident(ident)),
            Token::Opr(_) => None,
            Token::LeftDelimiter(_) => None,
            Token::RightDelimiter(_) => None,
            Token::Separator(_) => None,
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
    LeftDelimiter(LeftDelimiter),
    RightDelimiter(RightDelimiter),
    Ast(AstData),
    Separator(Separator),
}

impl std::fmt::Debug for PreAst {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PreAst::Keyword(slf) => slf.fmt(f),
            PreAst::Opr(slf) => slf.fmt(f),
            PreAst::LeftDelimiter(slf) => slf.fmt(f),
            PreAst::RightDelimiter(slf) => slf.fmt(f),
            PreAst::Ast(slf) => slf.fmt(f),
            PreAst::Separator(slf) => slf.fmt(f),
        }
    }
}

impl From<Token> for PreAst {
    fn from(token: Token) -> Self {
        match token {
            Token::Keyword(kw) => PreAst::Keyword(kw),
            Token::Ident(ident) => PreAst::Ast(AstData::Ident(ident)),
            Token::Opr(opr) => PreAst::Opr(opr),
            Token::Literal(lit) => PreAst::Ast(AstData::Literal(lit)),
            Token::LeftDelimiter(delimiter) => PreAst::LeftDelimiter(delimiter),
            Token::RightDelimiter(delimiter) => PreAst::RightDelimiter(delimiter),
            Token::Separator(separator) => PreAst::Separator(separator),
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
                Some(`let`),
                Some(Ident(`hello`)),
                Some(`=`),
                Some(Ident(`world`)),
                Some(`+(add)`),
                Some(Ident(`humans`)),
            ]
        "#]],
    );
}
