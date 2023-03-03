#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[enum_class::from_variants]
pub enum ExprDisambiguation {
    IndexOrComposeWithList(IndexOrComposeWithListExprDisambiguation),
    UnveilOrComposeWithOption(UnveilOrComposeWithOptionExprDisambiguation),
    ExplicitApplicationOrRitchieCall(ApplicationOrRitchieCallExprDisambiguation),
    List(ListExprDisambiguation),
    Trivial,
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
pub enum ListExprDisambiguation {
    NewList,
    ListFunctor,
}
