use super::*;
use husky_entity_route::{EntityKind, RangedEntityRoute};
use husky_opn_syntax::{
    BinaryOpr, BinaryPureClosedOpr, Bracket, ListEndAttr, ListStartAttr, PrefixOpr, RawSuffixOpr,
};
use husky_pattern_syntax::RawPattern;
use husky_primitive_literal_syntax::RawLiteralData;
use husky_text::*;
use husky_token::SpecialToken;
use husky_word::{CustomIdentifier, WordOpr, WordPattern};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HuskyAtomVariant {
    EntityRoute {
        route: Ty,
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
        opt_this_ty: Option<Ty>,
        opt_this_liason: Option<ParameterModifier>,
    },
    ThisField {
        field_ident: RangedCustomIdentifier,
        opt_this_ty: Option<Ty>,
        opt_this_liason: Option<ParameterModifier>,
        opt_field_ty: Option<RangedEntityRoute>,
        field_liason: MemberModifier,
    },
    Unrecognized(CustomIdentifier),
    PrimitiveLiteral(RawLiteralData),
    Binary(BinaryOpr),
    Prefix(PrefixOpr),
    Suffix(RawSuffixOpr),
    FieldAccess(Option<RangedCustomIdentifier>),
    ListStart(Bracket, ListStartAttr),
    ListEnd(Bracket, ListEndAttr),
    ListItem,
    LambdaHead(Vec<(RangedCustomIdentifier, Option<RangedEntityRoute>)>),
    SilentEnd,
    Be,
    BePattern(RawPattern),
    WordPattern {
        patt: WordPattern,
    },
}

pub type LambdaHead = Vec<(Identifier, Option<RangedEntityRoute>)>;

impl From<BinaryOpr> for HuskyAtomVariant {
    fn from(opr: BinaryOpr) -> Self {
        Self::Binary(opr)
    }
}

impl From<PrefixOpr> for HuskyAtomVariant {
    fn from(opr: PrefixOpr) -> Self {
        Self::Prefix(opr)
    }
}

impl From<RawSuffixOpr> for HuskyAtomVariant {
    fn from(opr: RawSuffixOpr) -> Self {
        Self::Suffix(opr)
    }
}

impl From<SpecialToken> for HuskyAtomVariant {
    fn from(special: SpecialToken) -> Self {
        match special {
            SpecialToken::BinaryOpr(opr) => opr.into(),
            SpecialToken::BinaryOpr(BinaryOpr::ScopeResolution)
            | SpecialToken::Colon
            | SpecialToken::Vertical
            | SpecialToken::Ambersand
            | SpecialToken::Exclamation
            | SpecialToken::DoubleVertical
            | SpecialToken::BinaryOpr(BinaryOpr::Curry)
            | SpecialToken::Bra(_)
            | SpecialToken::Ket(_)
            | SpecialToken::Minus
            | SpecialToken::FieldAccess => {
                p!(special);
                panic!()
            }
            SpecialToken::LAngle => BinaryOpr::Comparison(BinaryComparisonOpr::Less).into(),
            SpecialToken::RAngle => panic!("should check whether this is a `>>`"),
            SpecialToken::BitNot => PrefixOpr::BitNot.into(),
            SpecialToken::DoubleExclamation => PrefixOpr::Move.into(),
            SpecialToken::BinaryOpr(BinaryOpr::PureClosed(BinaryPureClosedOpr::RemEuclid)) => {
                BinaryOpr::PureClosed(BinaryPureClosedOpr::RemEuclid).into()
            }
            SpecialToken::Incr => HuskyAtomVariant::Suffix(RawSuffixOpr::Incr),
            SpecialToken::Decr => HuskyAtomVariant::Suffix(RawSuffixOpr::Decr),
            SpecialToken::Comma => HuskyAtomVariant::ListItem,
            SpecialToken::Semicolon => HuskyAtomVariant::SilentEnd,
            SpecialToken::XmlKet => todo!(),
            SpecialToken::DeriveAssign => todo!(),
            SpecialToken::At => todo!(),
            SpecialToken::QuestionMark => todo!(),
        }
    }
}

impl From<WordOpr> for HuskyAtomVariant {
    fn from(word_opr: WordOpr) -> Self {
        match word_opr {
            WordOpr::And => {
                HuskyAtomVariant::Binary(BinaryOpr::ShortcuitLogic(BinaryShortcuitLogicOpr::And))
            }
            WordOpr::Or => {
                HuskyAtomVariant::Binary(BinaryOpr::ShortcuitLogic(BinaryShortcuitLogicOpr::Or))
            }
            WordOpr::As => panic!(),
            WordOpr::Be => todo!(),
        }
    }
}

impl From<RawLiteralData> for HuskyAtomVariant {
    fn from(lit: RawLiteralData) -> Self {
        Self::PrimitiveLiteral(lit)
    }
}

// impl From<i32> for AtomVariant {
//     fn from(i: i32) -> Self {
//         RawLiteralData::I32(i).into()
//     }
// }

// impl From<f32> for AtomVariant {
//     fn from(f: f32) -> Self {
//         PrimitiveValueData::F32(f).into()
//     }
// }
