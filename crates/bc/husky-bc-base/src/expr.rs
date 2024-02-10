use crate::var::SymbolId;

pub enum BcExpr {
    Svar(SymbolId),
    Hvar(SymbolId),
    ForAll { symbol: SymbolId },
    Lambda(SymbolId),
    Composite(BcExprs),
}

pub type BcExprs = Vec<BcExpr>;
