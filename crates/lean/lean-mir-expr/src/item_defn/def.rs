use super::*;
use crate::{expr::LnMirExprIdx, stmt::LnMirStmtIdxRange, tactic::LnMirTacticIdxRange};

pub struct LnDefParameter {
    pub ident: LnIdent,
    pub ty: LnMirExprIdx,
}

#[enum_class::from_variants]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LnMirDefBody {
    Expr(LnMirExprIdx),
    Tactics(LnMirTacticIdxRange),
    Stmts(LnMirStmtIdxRange),
}
