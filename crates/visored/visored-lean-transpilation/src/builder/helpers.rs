use super::*;
use lean_mir_expr::tactic::{LnMirTacticData, LnMirTacticIdxRange};

impl<'a, S> VdLeanTranspilationBuilder<'a, S>
where
    S: IsVdLeanTranspilationScheme,
{
    pub fn default_tactics(&mut self) -> LnMirTacticIdxRange {
        let tactic = self.alloc_tactic(LnMirTacticData::Obvious);
        LnMirTacticIdxRange::new_single(tactic)
    }

    pub fn default_tactic_data(&mut self) -> LnMirTacticData {
        LnMirTacticData::Obvious
    }
}
