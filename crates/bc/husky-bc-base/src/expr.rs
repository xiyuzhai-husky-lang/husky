use crate::symbol::SymbolId;

pub enum BcExpr {
    Atomic(SymbolId),
    Composite(BcExprs),
}

pub type BcExprs = Vec<BcExpr>;
