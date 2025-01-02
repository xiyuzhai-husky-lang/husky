pub mod linear;

use crate::{
    expr::VdMirExprIdx,
    hypothesis::constructor::VdMirHypothesisConstructor,
    region::{VdMirExprRegionDataMut, VdMirExprRegionDataRef},
    stmt::{VdMirStmtIdx, VdMirStmtIdxRange},
    *,
};

pub trait IsVdMirTacticElaborator: std::fmt::Debug {
    fn elaborate_stmts_ext(
        self,
        stmts: VdMirStmtIdxRange,
        hypothesis_constructor: &mut VdMirHypothesisConstructor,
    );
    fn elaborate_stmt_ext(
        self,
        stmt: VdMirStmtIdx,
        hypothesis_constructor: &mut VdMirHypothesisConstructor,
    );
    fn elaborate_expr_ext(
        self,
        expr: VdMirExprIdx,
        hypothesis_constructor: &mut VdMirHypothesisConstructor,
    );
}

pub type VdMirTrivialElaborator = self::linear::VdMirSequentialElaborator<()>;
