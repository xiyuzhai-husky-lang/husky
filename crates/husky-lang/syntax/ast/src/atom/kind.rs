use scope::ScopeKind;
use token::Special;
use word::CustomIdentifier;

use super::*;
use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AtomKind {
    Scope(ScopeId, ScopeKind),
    Variable(Identifier),
    Literal(syntax_types::Literal),
    Binary(BinaryOpr),
    Prefix(PrefixOpr),
    Suffix(SuffixOpr),
    ListStart(Bracket, ListStartAttr),
    ListEnd(Bracket, ListEndAttr),
    ListItem,
    LambdaHead(Vec<(CustomIdentifier, Option<ScopeId>)>),
}

pub type LambdaHead = Vec<(Identifier, Option<ScopeId>)>;

impl From<BinaryOpr> for AtomKind {
    fn from(opr: BinaryOpr) -> Self {
        Self::Binary(opr)
    }
}

impl From<PrefixOpr> for AtomKind {
    fn from(opr: PrefixOpr) -> Self {
        Self::Prefix(opr)
    }
}

impl From<SuffixOpr> for AtomKind {
    fn from(opr: SuffixOpr) -> Self {
        Self::Suffix(opr)
    }
}

impl From<&Special> for AtomKind {
    fn from(special: &Special) -> Self {
        match special {
            Special::DoubleColon
            | Special::Colon
            | Special::Vertical
            | Special::Ambersand
            | Special::Exclamation
            | Special::DoubleVertical
            | Special::LightArrow
            | Special::HeavyArrow
            | Special::LPar
            | Special::LBox
            | Special::LCurl
            | Special::RCurl
            | Special::RBox
            | Special::RPar
            | Special::SubOrMinus
            | Special::MemberAccess
            | Special::Assign
            | Special::AddAssign
            | Special::SubAssign
            | Special::MultAssign
            | Special::DivAssign => panic!(),
            Special::LAngle => AtomKind::Binary(BinaryOpr::Less),
            Special::Leq => AtomKind::Binary(BinaryOpr::Leq),
            Special::RAngle => AtomKind::Binary(BinaryOpr::Greater),
            Special::Geq => AtomKind::Binary(BinaryOpr::Geq),
            Special::Neq => AtomKind::Binary(BinaryOpr::Neq),
            Special::Eq => AtomKind::Binary(BinaryOpr::Eq),
            Special::LShift => AtomKind::Binary(BinaryOpr::LShift),
            Special::RShift => AtomKind::Binary(BinaryOpr::RShift),
            Special::Add => AtomKind::Binary(BinaryOpr::Add),
            Special::Mult => AtomKind::Binary(BinaryOpr::Mult),
            Special::Div => AtomKind::Binary(BinaryOpr::Div),
            Special::Power => AtomKind::Binary(BinaryOpr::Power),
            Special::And => AtomKind::Binary(BinaryOpr::And),
            Special::BitNot => AtomKind::Prefix(PrefixOpr::BitNot),
            Special::Modulo => AtomKind::Binary(BinaryOpr::Modulo),
            Special::Incr => AtomKind::Suffix(SuffixOpr::Incr),
            Special::Decr => AtomKind::Suffix(SuffixOpr::Decr),
            Special::Comma => AtomKind::ListItem,
        }
    }
}

impl From<Identifier> for AtomKind {
    fn from(ident: Identifier) -> Self {
        Self::Variable(ident)
    }
}

impl From<&Identifier> for AtomKind {
    fn from(ident: &Identifier) -> Self {
        Self::Variable(*ident)
    }
}

impl From<Literal> for AtomKind {
    fn from(lit: Literal) -> Self {
        Self::Literal(lit)
    }
}

impl From<&i32> for AtomKind {
    fn from(i: &i32) -> Self {
        Literal::I32Literal(*i).into()
    }
}

impl From<&f32> for AtomKind {
    fn from(f: &f32) -> Self {
        Literal::F32Literal(*f).into()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum BracketOpr {
    Call,        // $0($1,*)
    Index,       // $0[$1,*]
    Curl,        // $0{$1,*}
    ParIndex,    // $0[$1,*%]
    DoubleIndex, // $0[[$1,*]]
}
