use super::*;

#[salsa::interned]
pub struct PlotClassTerm {
    #[return_ref]
    data: PlotClassTermData,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PlotClassTermData {
    Literal(PlotClass),
    SymbolicVariable(EthSymbolicVariable),
    LambdaVariable(EthLambdaVariable),
    Group(Vec<PlotClassTerm>),
}
