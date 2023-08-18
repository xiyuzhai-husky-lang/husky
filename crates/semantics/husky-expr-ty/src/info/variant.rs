use crate::*;

#[derive(Debug, PartialEq, Eq)]
#[salsa::debug_with_db(db = ExprTypeDb)]
#[enum_class::from_variants]
pub enum SynExprDisambiguation {
    Trivial,
    IndexOrComposeWithList(IndexOrComposeWithListExprDisambiguation),
    UnveilOrComposeWithOption(UnveilOrComposeWithOptionExprDisambiguation),
    UnwrapOrComposeWithNot(UnwrapOrComposeWithNotExprDisambiguation),
    ApplicationOrFunctionCall(ApplicationOrFunctionCallExprDisambiguation),
    TypePath(TypePathDisambiguation),
    List(ListExprDisambiguation),
    Application(ApplicationDisambiguation),
    Tilde(TildeDisambiguation),
    FieldDispatch(FluffyFieldDispatch),
    MethodCallOrApplication(MethodCallOrApplicationDisambiguation),
    StaticDispatch(StaticDispatch),
    FunctionCall {
        ritchie_kind: RitchieKind,
        ritchie_parameter_argument_matches: ExprTypeResult<RitchieParameterArgumentMatches>,
    },
}

impl SynExprDisambiguation {
    pub(crate) fn list_expr_disambiguation(&self) -> Option<ListExprDisambiguation> {
        match self {
            SynExprDisambiguation::List(disambiguation) => Some(*disambiguation),
            _ => None,
        }
    }
}

/// disambiguate between `unveil` and compose with `List`
#[derive(Debug, PartialEq, Eq)]
#[salsa::debug_with_db(db = ExprTypeDb)]
pub enum IndexOrComposeWithListExprDisambiguation {
    Index(FluffyIndexDispatch),
    ComposeWithList,
}

/// disambiguate between `unveil` and compose with `Option`
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = ExprTypeDb)]
pub enum UnveilOrComposeWithOptionExprDisambiguation {
    Unveil,
    ComposeWithOption,
}

/// disambiguate between `unwrap` and compose with `Not`
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = ExprTypeDb)]
pub enum UnwrapOrComposeWithNotExprDisambiguation {
    Unwrap,
    ComposeWithNot,
}

#[derive(Debug, PartialEq, Eq)]
#[salsa::debug_with_db(db = ExprTypeDb)]
pub enum ApplicationOrFunctionCallExprDisambiguation {
    Application,
    FnCall {
        ritchie_parameter_argument_matches: RitchieParameterArgumentMatches,
    },
    GnCall {
        ritchie_parameter_argument_matches: RitchieParameterArgumentMatches,
    },
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = ExprTypeDb)]
pub struct ApplicationDisambiguation {
    shift: u8,
}

impl ApplicationDisambiguation {
    pub fn new(shift: u8) -> Self {
        Self { shift }
    }

    pub fn shift(self) -> u8 {
        self.shift
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = ExprTypeDb)]
pub enum ListExprDisambiguation {
    NewList,
    ListFunctor,
    ArrayFunctor,
}

#[derive(Debug, PartialEq, Eq)]
#[salsa::debug_with_db(db = ExprTypeDb)]
pub enum MethodCallOrApplicationDisambiguation {
    MethodCall {
        method_dispatch: FluffyMethodDispatch,
        ritchie_parameter_argument_matches: ExprTypeResult<RitchieParameterArgumentMatches>,
    },
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = ExprTypeDb)]
pub enum TildeDisambiguation {
    BitNot,
    Leash,
}
