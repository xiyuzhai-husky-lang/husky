use super::*;
use husky_entity_route::{EntityKind, RangedEntityRoute};
use husky_text::*;
use husky_token::{PrimitiveLiteralData, SpecialToken};
use vm::{BinaryOpr, Bracket, ListEndAttr, ListStartAttr, PrefixOpr, PureBinaryOpr, SuffixOpr};
use word::{CustomIdentifier, WordOpr};

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
    ThisValue {
        opt_this_ty: Option<EntityRoutePtr>,
        opt_this_liason: Option<ParameterLiason>,
    },
    ThisField {
        field_ident: RangedCustomIdentifier,
        opt_this_ty: Option<EntityRoutePtr>,
        opt_this_liason: Option<ParameterLiason>,
        opt_field_ty: Option<RangedEntityRoute>,
        field_liason: MemberLiason,
    },
    Unrecognized(CustomIdentifier),
    PrimitiveLiteral(PrimitiveLiteralData),
    Binary(BinaryOpr),
    Prefix(PrefixOpr),
    Suffix(SuffixOpr),
    FieldAccess(RangedCustomIdentifier),
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

impl From<SpecialToken> for AtomVariant {
    fn from(special: SpecialToken) -> Self {
        match special {
            SpecialToken::DoubleColon
            | SpecialToken::Colon
            | SpecialToken::Vertical
            | SpecialToken::Ambersand
            | SpecialToken::Exclamation
            | SpecialToken::DoubleVertical
            | SpecialToken::LightArrow
            | SpecialToken::HeavyArrow
            | SpecialToken::LPar
            | SpecialToken::LBox
            | SpecialToken::LCurl
            | SpecialToken::RCurl
            | SpecialToken::RBox
            | SpecialToken::RPar
            | SpecialToken::SubOrMinus
            | SpecialToken::MemberAccess => {
                p!(special);
                panic!()
            }
            SpecialToken::Assign => BinaryOpr::Assign(None).into(),
            SpecialToken::AddAssign => BinaryOpr::Assign(Some(PureBinaryOpr::Add)).into(),
            SpecialToken::SubAssign => BinaryOpr::Assign(Some(PureBinaryOpr::Sub)).into(),
            SpecialToken::MulAssign => BinaryOpr::Assign(Some(PureBinaryOpr::Mul)).into(),
            SpecialToken::DivAssign => BinaryOpr::Assign(Some(PureBinaryOpr::Div)).into(),
            SpecialToken::BitOrAssign => BinaryOpr::Assign(Some(PureBinaryOpr::BitOr)).into(),
            SpecialToken::BitAndAssign => BinaryOpr::Assign(Some(PureBinaryOpr::BitAnd)).into(),
            SpecialToken::LAngle => BinaryOpr::Pure(PureBinaryOpr::Less).into(),
            SpecialToken::Leq => BinaryOpr::Pure(PureBinaryOpr::Leq).into(),
            SpecialToken::RAngle => BinaryOpr::Pure(PureBinaryOpr::Greater).into(),
            SpecialToken::Geq => BinaryOpr::Pure(PureBinaryOpr::Geq).into(),
            SpecialToken::Neq => BinaryOpr::Pure(PureBinaryOpr::Neq).into(),
            SpecialToken::Eq => BinaryOpr::Pure(PureBinaryOpr::Eq).into(),
            SpecialToken::Shl => BinaryOpr::Pure(PureBinaryOpr::Shl).into(),
            SpecialToken::Shr => BinaryOpr::Pure(PureBinaryOpr::Shr).into(),
            SpecialToken::Add => BinaryOpr::Pure(PureBinaryOpr::Add).into(),
            SpecialToken::Star => BinaryOpr::Pure(PureBinaryOpr::Mul).into(),
            SpecialToken::Div => BinaryOpr::Pure(PureBinaryOpr::Div).into(),
            SpecialToken::Power => BinaryOpr::Pure(PureBinaryOpr::Power).into(),
            SpecialToken::And => BinaryOpr::Pure(PureBinaryOpr::And).into(),
            SpecialToken::BitNot => PrefixOpr::BitNot.into(),
            SpecialToken::DoubleExclamation => PrefixOpr::Move.into(),
            SpecialToken::Modulo => BinaryOpr::Pure(PureBinaryOpr::RemEuclid).into(),
            SpecialToken::Incr => AtomVariant::Suffix(SuffixOpr::Incr),
            SpecialToken::Decr => AtomVariant::Suffix(SuffixOpr::Decr),
            SpecialToken::Comma => AtomVariant::ListItem,
            SpecialToken::Semicolon => AtomVariant::SilentEnd,
            SpecialToken::XmlKet => todo!(),
            SpecialToken::DeriveAssign => todo!(),
            SpecialToken::At => todo!(),
            SpecialToken::QuestionMark => todo!(),
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

impl From<PrimitiveLiteralData> for AtomVariant {
    fn from(lit: PrimitiveLiteralData) -> Self {
        Self::PrimitiveLiteral(lit)
    }
}

// impl From<i32> for AtomVariant {
//     fn from(i: i32) -> Self {
//         PrimitiveLiteralData::I32(i).into()
//     }
// }

// impl From<f32> for AtomVariant {
//     fn from(f: f32) -> Self {
//         PrimitiveValueData::F32(f).into()
//     }
// }
