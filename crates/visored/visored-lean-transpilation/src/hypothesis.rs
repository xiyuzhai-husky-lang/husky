use super::*;
use lean_entity_path::theorem::LnTheoremPath;
use lean_mir_expr::{
    expr::{LnMirExprData, LnMirExprEntry},
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
        let new_hypotheses = hypothesis_chunk.new_hypotheses();
        let main_hypothesis = hypothesis_chunk.main_hypothesis();
        for hypothesis in new_hypotheses {
            self.build_hypothesis_tactics(hypothesis, ln_tactics);
        }
        if !new_hypotheses.contains(main_hypothesis) {
            self.build_old_main_hypothesis_tactics(main_hypothesis, ln_tactics);
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
            VdMirHypothesisConstruction::TermEquivalent { hypothesis } => {
                let ad_hoc_tactic_data = self.ad_hoc_tactic_data("term_equivalent");
                self.alloc_tactics([ad_hoc_tactic_data])
            }
            VdMirHypothesisConstruction::CommRing => {
                let ad_hoc_tactic_data = self.ad_hoc_tactic_data("comm_ring");
                self.alloc_tactics([ad_hoc_tactic_data])
            }
            VdMirHypothesisConstruction::LetAssigned => {
                let ad_hoc_tactic_data = self.ad_hoc_tactic_data("let_assigned");
                self.alloc_tactics([ad_hoc_tactic_data])
            }
            VdMirHypothesisConstruction::LitnumReduce => {
                let ad_hoc_tactic_data = self.ad_hoc_tactic_data("litnum_reduce");
                self.alloc_tactics([ad_hoc_tactic_data])
            }
        };
        let construction = self.alloc_expr(LnMirExprEntry::new(
            LnMirExprData::By {
                tactics: construction_tactics,
            },
            None,
        ));
        let ident = self.mangle_hypothesis();
        ln_tactics.push(LnMirTacticData::Have {
            ident,
            ty: Some(hypothesis_entry.expr().to_lean(self)),
            construction,
        });
    }

    fn build_old_main_hypothesis_tactics(
        &mut self,
        main_hypothesis: VdMirHypothesisIdx,
        ln_tactics: &mut Vec<LnMirTacticData>,
    ) {
        let ad_hoc_tactic_data = self.ad_hoc_tactic_data("old_main_hypothesis");
        let construction_tactics = self.alloc_tactics([ad_hoc_tactic_data]);
        let construction = self.alloc_expr(LnMirExprEntry::new(
            LnMirExprData::By {
                tactics: construction_tactics,
            },
            None,
        ));
        ln_tactics.push(LnMirTacticData::Have {
            ident: self.mangle_hypothesis(),
            ty: Some(
                self.hypothesis_arena()[main_hypothesis]
                    .expr()
                    .to_lean(self),
            ),
            construction,
        });
    }
}
