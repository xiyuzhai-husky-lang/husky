use crate::*;
use lean_mir_expr::tactic::{LnMirTacticData, LnMirTacticIdxRange};
use visored_mir_expr::tactic::{
    elaboration::VdMirTacticElaboration, VdMirTacticIdx, VdMirTacticIdxRange,
};

impl<'db, S> VdTranspileToLean<S, LnMirTacticIdxRange> for VdMirTacticIdxRange
where
    S: IsVdLeanTranspilationScheme,
{
    fn to_lean(self, builder: &mut VdLeanTranspilationBuilder<S>) -> LnMirTacticIdxRange {
        let mut tactics = vec![];
        for tactic in self {
            tactics.push(builder.build_tactic(tactic));
        }
        builder.alloc_tactics(tactics)
    }
}

impl<'a, S> VdLeanTranspilationBuilder<'a, S>
where
    S: IsVdLeanTranspilationScheme,
{
    pub(crate) fn build_tactic(&mut self, tactic: VdMirTacticIdx) -> LnMirTacticData {
        let entry = &self.tactic_arena()[tactic];
        self.debug_tactic(tactic, format!("{:?}", entry.data()));
        match entry.elaboration_tracker().conclusion() {
            Some(_) => todo!(),
            None => todo!(),
        }
    }
}
