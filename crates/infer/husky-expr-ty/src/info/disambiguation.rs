use husky_ty_expectation::TypePathDisambiguation;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[enum_class::from_variants]
pub enum ExprDisambiguation {
    IndexOrComposeWithList(IndexOrComposeWithListExprDisambiguation),
    UnveilOrComposeWithOption(UnveilOrComposeWithOptionExprDisambiguation),
    ExplicitApplicationOrRitchieCall(ApplicationOrRitchieCallExprDisambiguation),
    TypePath(TypePathDisambiguation),
    List(ListExprDisambiguation),
    ExplicitApplication(ExplicitApplicationDisambiguation),
    Trivial,
}

impl ExprDisambiguation {
    pub(crate) fn list(self) -> Option<ListExprDisambiguation> {
        match self {
            ExprDisambiguation::List(disambiguation) => Some(disambiguation),
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
