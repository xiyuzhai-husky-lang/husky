use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub enum RuntimeExprSymbol {
    Current(CurrentRuntimeExprSymbol),
    Inherited(InheritedRuntimeExprSymbol),
}

#[derive(Debug, PartialEq, Eq)]
pub enum InheritedRuntimeExprSymbol {
    Term(InheritedTermExprSymbol),
}

pub type InheritedRuntimeExprSymbolIdx = ();

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InheritedRuntimeExprSymbolKind {}

#[derive(Debug, PartialEq, Eq)]
pub enum CurrentRuntimeExprSymbol {
    Term(CurrentTermExprSymbol),
}

pub type CurrentRuntimeExprSymbolIdx = ();

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CurrentRuntimeExprSymbolKind {}
