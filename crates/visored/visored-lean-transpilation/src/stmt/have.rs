use lean_mir_expr::tactic::LnMirTacticIdxRange;

use super::*;

impl<'a, S> VdLeanTranspilationBuilder<'a, S>
where
    S: IsVdLeanTranspilationScheme,
{
    pub(super) fn build_have_tactics(&mut self, stmt: VdMirStmtIdx) -> LnMirTacticIdxRange {
        match self.stmt_arena()[stmt].elaboration_tracker().conclusion() {
            Some(_) => todo!(),
            None => self.default_tactics(),
        }
    }
}
