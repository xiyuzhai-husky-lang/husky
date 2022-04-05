use entity_route::{RangedScope, RawEntityKind, ScopeKind};
use text::Row;
use token::Special;
use vm::{BinaryOpr, PrimitiveValue, PureBinaryOpr};
use word::CustomIdentifier;

use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AtomKind {
    Scope {
        scope: EntityRoutePtr,
        kind: RawEntityKind,
    },
    Variable {
        varname: CustomIdentifier,
        init_row: Row,
    },
    This {
        ty: Option<EntityRoutePtr>,
    },
    Unrecognized(CustomIdentifier),
    Literal(PrimitiveValue),
    Binary(BinaryOpr),
    Prefix(PrefixOpr),
    Suffix(SuffixOpr),
    ListStart(Bracket, ListStartAttr),
    ListEnd(Bracket, ListEndAttr),
    ListItem,
    LambdaHead(Vec<(CustomIdentifier, Option<RangedScope>)>),
}

pub type LambdaHead = Vec<(Identifier, Option<RangedScope>)>;

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

impl From<Special> for AtomKind {
    fn from(special: Special) -> Self {
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
            | Special::MemberAccess => panic!(),
            Special::Assign => BinaryOpr::Assign(None).into(),
            Special::AddAssign => BinaryOpr::Assign(Some(PureBinaryOpr::Add)).into(),
            Special::SubAssign => BinaryOpr::Assign(Some(PureBinaryOpr::Sub)).into(),
            Special::MulAssign => BinaryOpr::Assign(Some(PureBinaryOpr::Mul)).into(),
            Special::DivAssign => BinaryOpr::Assign(Some(PureBinaryOpr::Div)).into(),
            Special::LAngle => BinaryOpr::Pure(PureBinaryOpr::Less).into(),
            Special::Leq => BinaryOpr::Pure(PureBinaryOpr::Leq).into(),
            Special::RAngle => BinaryOpr::Pure(PureBinaryOpr::Greater).into(),
            Special::Geq => BinaryOpr::Pure(PureBinaryOpr::Geq).into(),
            Special::Neq => BinaryOpr::Pure(PureBinaryOpr::Neq).into(),
            Special::Eq => BinaryOpr::Pure(PureBinaryOpr::Eq).into(),
            Special::Shl => BinaryOpr::Pure(PureBinaryOpr::Shl).into(),
            Special::Shr => BinaryOpr::Pure(PureBinaryOpr::Shr).into(),
            Special::Add => BinaryOpr::Pure(PureBinaryOpr::Add).into(),
            Special::Mul => BinaryOpr::Pure(PureBinaryOpr::Mul).into(),
            Special::Div => BinaryOpr::Pure(PureBinaryOpr::Div).into(),
            Special::Power => BinaryOpr::Pure(PureBinaryOpr::Power).into(),
            Special::And => BinaryOpr::Pure(PureBinaryOpr::And).into(),
            Special::BitNot => PrefixOpr::BitNot.into(),
            Special::Modulo => BinaryOpr::Pure(PureBinaryOpr::RemEuclid).into(),
            Special::Incr => AtomKind::Suffix(SuffixOpr::Incr),
            Special::Decr => AtomKind::Suffix(SuffixOpr::Decr),
            Special::Comma => AtomKind::ListItem,
        }
    }
}

impl From<PrimitiveValue> for AtomKind {
    fn from(lit: PrimitiveValue) -> Self {
        Self::Literal(lit)
    }
}

impl From<i32> for AtomKind {
    fn from(i: i32) -> Self {
        PrimitiveValue::I32(i).into()
    }
}

impl From<f32> for AtomKind {
    fn from(f: f32) -> Self {
        PrimitiveValue::F32(f).into()
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
