use crate::*;
use husky_term::{TraitForTypeMethodFnCard, TypeMethodFnCard};
use husky_ty_expectation::TypePathDisambiguation;

#[derive(Debug, PartialEq, Eq)]
#[enum_class::from_variants]
pub enum ExprDisambiguation {
    IndexOrComposeWithList(IndexOrComposeWithListExprDisambiguation),
    UnveilOrComposeWithOption(UnveilOrComposeWithOptionExprDisambiguation),
    ExplicitApplicationOrRitchieCall(ApplicationOrRitchieCallExprDisambiguation),
    TypePath(TypePathDisambiguation),
    List(ListExprDisambiguation),
    ExplicitApplication(ExplicitApplicationDisambiguation),
    Tilde(TildeDisambiguation),
    Method(FluffyMethodDisambiguation),
    Trivial,
    Field(FluffyFieldDisambiguation),
}

impl ExprDisambiguation {
    pub(crate) fn list_expr_disambiguation(&self) -> Option<ListExprDisambiguation> {
        match self {
            ExprDisambiguation::List(disambiguation) => Some(*disambiguation),
            _ => None,
        }
    }
}

/// disambiguate between `unveil` and compose with `List`
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum IndexOrComposeWithListExprDisambiguation {
    Index,
    ComposeWithList,
}

/// disambiguate between `unveil` and compose with `Option`
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum UnveilOrComposeWithOptionExprDisambiguation {
    Unveil,
    ComposeWithOption,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ApplicationOrRitchieCallExprDisambiguation {
    Application,
    RitchieCall,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ExplicitApplicationDisambiguation {
    shift: u8,
}

impl ExplicitApplicationDisambiguation {
    pub fn new(shift: u8) -> Self {
        Self { shift }
    }

    pub fn shift(self) -> u8 {
        self.shift
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ListExprDisambiguation {
    NewList,
    ListFunctor,
    ArrayFunctor,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TildeDisambiguation {
    BitNot,
    Leash,
}

// #[derive(Debug, PartialEq, Eq, Clone, Copy)]
// pub struct MethodCallDisambiguation(MethodCard);

// impl From<MethodCard> for MethodCallDisambiguation {
//     fn from(value: MethodCard) -> Self {
//         MethodCallDisambiguation(value)
//     }
// }

// impl From<TypeMethodFnCard> for MethodCallDisambiguation {
//     fn from(value: TypeMethodFnCard) -> Self {
//         MethodCallDisambiguation(value.into())
//     }
// }

// impl From<TraitForTypeMethodFnCard> for MethodCallDisambiguation {
//     fn from(value: TraitForTypeMethodFnCard) -> Self {
//         MethodCallDisambiguation(value.into())
//     }
// }

// #[derive(Debug, PartialEq, Eq, Clone, Copy)]
// pub enum FieldOrMemoDisambiguation {
//     Field,
//     Memo,
// }
