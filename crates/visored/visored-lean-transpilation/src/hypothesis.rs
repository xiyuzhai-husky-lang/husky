use super::*;
use lean_entity_path::theorem::LnTheoremPath;
use lean_mir_expr::{
    expr::LnMirExprData,
    tactic::{LnMirTacticData, LnMirTacticIdxRange},
};
use visored_entity_path::theorem::VdTheoremPath;
use visored_mir_expr::{
    coercion::VdMirCoercion,
    hypothesis::{
        chunk::VdMirHypothesisChunk, construction::VdMirHypothesisConstruction, VdMirHypothesisIdx,
    },
};

impl<'a, S> VdLeanTranspilationBuilder<'a, S>
where
    S: IsVdLeanTranspilationScheme,
{
    pub(crate) fn build_hypothesis_chunk_tactics(
        &mut self,
        hypothesis_chunk: VdMirHypothesisChunk,
        ln_tactics: &mut Vec<LnMirTacticData>,
    ) {
        for hypothesis in hypothesis_chunk.new_hypotheses() {
            self.build_hypothesis_tactics(hypothesis, ln_tactics);
        }
    }

    fn build_hypothesis_tactics(
        &mut self,
        hypothesis: VdMirHypothesisIdx,
        ln_tactics: &mut Vec<LnMirTacticData>,
    ) {
        let hypothesis_entry = &self.hypothesis_arena()[hypothesis];
        let construction_tactics = match hypothesis_entry.construction() {
            VdMirHypothesisConstruction::Sorry => {
                let default_tactic_data = self.default_tactic_data();
                self.alloc_tactics([default_tactic_data])
            }
            VdMirHypothesisConstruction::TermTrivial(b) => {
                let ad_hoc_tactic_data = self.ad_hoc_tactic_data("term_trivial");
                self.alloc_tactics([ad_hoc_tactic_data])
            }
            VdMirHypothesisConstruction::Apply {
                path,
                is_real_coercion,
            } => {
                match is_real_coercion {
                    VdMirCoercion::Trivial => (),
                    VdMirCoercion::Obvious(arena_idx) => todo!("handle this properly."),
                }
                self.alloc_tactics([LnMirTacticData::Apply {
                    path: match path {
                        VdTheoremPath::SquareNonnegative => LnTheoremPath::SquareNonnegative,
                    },
                }])
            }
            VdMirHypothesisConstruction::Assume => return,
            VdMirHypothesisConstruction::TermEquivalent {} => {
                let ad_hoc_tactic_data = self.ad_hoc_tactic_data("term_equivalent");
                self.alloc_tactics([ad_hoc_tactic_data])
            }
            VdMirHypothesisConstruction::CommRing => {
                let ad_hoc_tactic_data = self.ad_hoc_tactic_data("comm_ring");
                self.alloc_tactics([ad_hoc_tactic_data])
            }
        };
        let construction = self.alloc_expr(LnMirExprData::By {
            tactics: construction_tactics,
        });
        let ident = self.mangle_hypothesis();
        ln_tactics.push(LnMirTacticData::Have {
            ident,
            ty: hypothesis_entry.expr().to_lean(self),
            construction,
        });
    }
}
