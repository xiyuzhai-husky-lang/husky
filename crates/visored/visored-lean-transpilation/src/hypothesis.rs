use super::*;
use lean_entity_path::theorem::LnTheoremPath;
use lean_mir_expr::tactic::{LnMirTacticData, LnMirTacticIdxRange};
use visored_entity_path::theorem::VdTheoremPath;
use visored_mir_expr::{
    coercion::VdMirCoercion,
    hypothesis::{construction::VdMirHypothesisConstruction, VdMirHypothesisIdx},
};

impl<'a, S> VdLeanTranspilationBuilder<'a, S>
where
    S: IsVdLeanTranspilationScheme,
{
    pub(crate) fn build_hypothesis_tactics(
        &mut self,
        hypothesis: VdMirHypothesisIdx,
    ) -> LnMirTacticIdxRange {
        let tactics = self.build_hypothesis_tactics_data(hypothesis);
        self.alloc_tactics(tactics)
    }

    pub(super) fn build_hypothesis_tactics_data(
        &mut self,
        hypothesis: VdMirHypothesisIdx,
    ) -> Vec<LnMirTacticData> {
        match self.hypothesis_arena()[hypothesis].construction() {
            VdMirHypothesisConstruction::Sorry => vec![self.default_tactic_data()],
            VdMirHypothesisConstruction::Apply {
                path,
                is_real_coercion,
            } => {
                match is_real_coercion {
                    VdMirCoercion::Trivial => (),
                    VdMirCoercion::Obvious(arena_idx) => todo!("handle this properly."),
                }
                vec![LnMirTacticData::Apply {
                    path: match path {
                        VdTheoremPath::SquareNonnegative => LnTheoremPath::SquareNonnegative,
                    },
                }]
            }
            VdMirHypothesisConstruction::Assume => todo!(),
        }
    }
}
