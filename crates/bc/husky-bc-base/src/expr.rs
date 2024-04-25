use crate::var::SymbolId;

pub enum BcExpr {
    SymbolicVariable(SymbolId),
    HiddenVariable(SymbolId),
    ForAll { symbol: SymbolId },
    Lambda(SymbolId),
    Composite(BcExprs),
}

pub type BcExprs = Vec<BcExpr>;
