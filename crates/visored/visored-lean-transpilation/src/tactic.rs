use crate::*;
use lean_mir_expr::tactic::LnMirTacticIdxRange;
use visored_mir_expr::tactic::VdMirTacticIdxRange;

impl<S> VdTranspileToLean<S, LnMirTacticIdxRange> for VdMirTacticIdxRange
where
    S: IsVdLeanTranspilationScheme,
{
    fn to_lean(self, builder: &mut VdLeanTranspilationBuilder<S>) -> LnMirTacticIdxRange {
        todo!()
    }
}
