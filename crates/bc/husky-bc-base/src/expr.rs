use crate::symbol::SymbolId;

pub enum BcExpr {
    Symbol(SymbolId),
    Rune(SymbolId),
    ForAll { symbol: SymbolId },
    Lambda(SymbolId),
    Composite(BcExprs),
}

pub type BcExprs = Vec<BcExpr>;
