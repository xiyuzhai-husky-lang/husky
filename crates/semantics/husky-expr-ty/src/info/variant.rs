use crate::*;
use husky_ty_expectation::TypePathDisambiguation;

#[derive(Debug, PartialEq, Eq)]
#[enum_class::from_variants]
pub enum ExprDisambiguation {
    Trivial,
    IndexOrComposeWithList(IndexOrComposeWithListExprDisambiguation),
    UnveilOrComposeWithOption(UnveilOrComposeWithOptionExprDisambiguation),
    UnwrapOrComposeWithNot(UnwrapOrComposeWithNotExprDisambiguation),
    ExplicitApplicationOrFunctionCall(ApplicationOrFunctionCallExprDisambiguation),
    TypePath(TypePathDisambiguation),
    List(ListExprDisambiguation),
    ExplicitApplication(ExplicitApplicationDisambiguation),
    Tilde(TildeDisambiguation),
    Field(FluffyFieldDisambiguation),
    Method(FluffyMethodDisambiguation),
    ScopeResolution(ScopeResolutionDisambiguation),
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
#[derive(Debug, PartialEq, Eq)]
pub enum IndexOrComposeWithListExprDisambiguation {
    Index(FluffyIndexDisambiguation),
    ComposeWithList,
}

/// disambiguate between `unveil` and compose with `Option`
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum UnveilOrComposeWithOptionExprDisambiguation {
    Unveil,
    ComposeWithOption,
}

/// disambiguate between `unwrap` and compose with `Not`
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum UnwrapOrComposeWithNotExprDisambiguation {
    Unwrap,
    ComposeWithNot,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ApplicationOrFunctionCallExprDisambiguation {
    Application,
    RitchieCall,
    GnCall,
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
