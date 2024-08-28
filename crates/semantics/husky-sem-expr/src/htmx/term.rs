use super::*;

#[salsa::interned]
pub struct PlotClassTerm {
    #[return_ref]
    data: PlotClassTermData,
}

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PlotClassTermData {
    Literal(PlotClass),
    SymbolicVariable(EthSymbolicVariable),
    LambdaVariable(EthLambdaVariable),
    Group(Vec<PlotClassTerm>),
}
