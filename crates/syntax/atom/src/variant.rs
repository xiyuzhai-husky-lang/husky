use entity_route::{EntityKind, RangedEntityRoute};
use text::*;
use token::Special;
use vm::{
    BinaryOpr, Bracket, CopyableValue, InputLiason, ListEndAttr, ListStartAttr, PrefixOpr,
    PureBinaryOpr, SuffixOpr,
};
use word::{CustomIdentifier, WordOpr};

use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AtomVariant {
    EntityRoute {
        route: EntityRoutePtr,
        kind: EntityKind,
    },
    Variable {
        varname: CustomIdentifier,
        init_range: TextRange,
    },
    FrameVariable {
        varname: CustomIdentifier,
        init_range: TextRange,
    },
    ThisData {
        opt_ty: Option<EntityRoutePtr>,
        opt_contract: Option<InputLiason>,
    },
    Unrecognized(CustomIdentifier),
    PrimitiveLiteral(CopyableValue),
    Binary(BinaryOpr),
    Prefix(PrefixOpr),
    Suffix(SuffixOpr),
    ListStart(Bracket, ListStartAttr),
    ListEnd(Bracket, ListEndAttr),
    ListItem,
    LambdaHead(Vec<(RangedCustomIdentifier, Option<RangedEntityRoute>)>),
    SilentEnd,
}

pub type LambdaHead = Vec<(Identifier, Option<RangedEntityRoute>)>;

impl From<BinaryOpr> for AtomVariant {
    fn from(opr: BinaryOpr) -> Self {
        Self::Binary(opr)
    }
}

impl From<PrefixOpr> for AtomVariant {
    fn from(opr: PrefixOpr) -> Self {
        Self::Prefix(opr)
    }
}

impl From<SuffixOpr> for AtomVariant {
    fn from(opr: SuffixOpr) -> Self {
        Self::Suffix(opr)
    }
}

impl From<Special> for AtomVariant {
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
            | Special::MemberAccess => {
                p!(special);
                panic!()
            }
            Special::Assign => BinaryOpr::Assign(None).into(),
            Special::AddAssign => BinaryOpr::Assign(Some(PureBinaryOpr::Add)).into(),
            Special::SubAssign => BinaryOpr::Assign(Some(PureBinaryOpr::Sub)).into(),
            Special::MulAssign => BinaryOpr::Assign(Some(PureBinaryOpr::Mul)).into(),
            Special::DivAssign => BinaryOpr::Assign(Some(PureBinaryOpr::Div)).into(),
            Special::BitOrAssign => BinaryOpr::Assign(Some(PureBinaryOpr::BitOr)).into(),
            Special::BitAndAssign => BinaryOpr::Assign(Some(PureBinaryOpr::BitAnd)).into(),
            Special::LAngle => BinaryOpr::Pure(PureBinaryOpr::Less).into(),
            Special::Leq => BinaryOpr::Pure(PureBinaryOpr::Leq).into(),
            Special::RAngle => BinaryOpr::Pure(PureBinaryOpr::Greater).into(),
            Special::Geq => BinaryOpr::Pure(PureBinaryOpr::Geq).into(),
            Special::Neq => BinaryOpr::Pure(PureBinaryOpr::Neq).into(),
            Special::Eq => BinaryOpr::Pure(PureBinaryOpr::Eq).into(),
            Special::Shl => BinaryOpr::Pure(PureBinaryOpr::Shl).into(),
            Special::Shr => BinaryOpr::Pure(PureBinaryOpr::Shr).into(),
            Special::Add => BinaryOpr::Pure(PureBinaryOpr::Add).into(),
            Special::Star => BinaryOpr::Pure(PureBinaryOpr::Mul).into(),
            Special::Div => BinaryOpr::Pure(PureBinaryOpr::Div).into(),
            Special::Power => BinaryOpr::Pure(PureBinaryOpr::Power).into(),
            Special::And => BinaryOpr::Pure(PureBinaryOpr::And).into(),
            Special::BitNot => PrefixOpr::BitNot.into(),
            Special::DoubleExclamation => PrefixOpr::Move.into(),
            Special::Modulo => BinaryOpr::Pure(PureBinaryOpr::RemEuclid).into(),
            Special::Incr => AtomVariant::Suffix(SuffixOpr::Incr),
            Special::Decr => AtomVariant::Suffix(SuffixOpr::Decr),
            Special::Comma => AtomVariant::ListItem,
            Special::Semicolon => AtomVariant::SilentEnd,
            Special::XmlKet => todo!(),
        }
    }
}

impl From<WordOpr> for AtomVariant {
    fn from(word_opr: WordOpr) -> Self {
        match word_opr {
            WordOpr::And => AtomVariant::Binary(BinaryOpr::Pure(PureBinaryOpr::And)),
            WordOpr::Or => AtomVariant::Binary(BinaryOpr::Pure(PureBinaryOpr::Or)),
            WordOpr::As => panic!(),
        }
    }
}

impl From<CopyableValue> for AtomVariant {
    fn from(lit: CopyableValue) -> Self {
        Self::PrimitiveLiteral(lit)
    }
}

impl From<i32> for AtomVariant {
    fn from(i: i32) -> Self {
        CopyableValue::I32(i).into()
    }
}

impl From<f32> for AtomVariant {
    fn from(f: f32) -> Self {
        CopyableValue::F32(f).into()
    }
}
