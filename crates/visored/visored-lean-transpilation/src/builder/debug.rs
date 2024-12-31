use super::*;
use visored_mir_expr::hint::{VdMirHintIdx, VdMirTacticSource};

impl<'a, S> VdLeanTranspilationBuilder<'a, S>
where
    S: IsVdLeanTranspilationScheme,
{
    pub fn debug_tactic(&mut self, tactic: VdMirHintIdx, message: String) {
        let src = self.source_map[tactic];
        let range = match src {
            VdMirTacticSource::Clause(clause) => self.sem_clause_range_map[clause],
        };
        let text_offset_range = self.token_storage.token_idx_range_offset_range(range);
        text_offset_range.emit_to_stdout(self.input(), message);
    }
}
