use super::*;
use lean_entity_path::LnItemPath;
use lean_mir_expr::tactic::{LnMirTacticData, LnMirTacticIdxRange};

impl<'a, S> VdLeanTranspilationBuilder<'a, S>
where
    S: IsVdLeanTranspilationScheme,
{
    pub fn default_tactics(&mut self) -> LnMirTacticIdxRange {
        let tactic_data = self.default_tactic_data();
        let tactic = self.alloc_tactic(tactic_data);
        LnMirTacticIdxRange::new_single(tactic)
    }

    pub fn default_tactic_data(&mut self) -> LnMirTacticData {
        LnMirTacticData::Obvious
    }

    pub fn exact_unit(&mut self) -> LnMirTacticData {
        let unit = self.alloc_expr(LnMirExprData::ItemPath(LnItemPath::UNIT));
        LnMirTacticData::Exact { term: unit }
    }
}
