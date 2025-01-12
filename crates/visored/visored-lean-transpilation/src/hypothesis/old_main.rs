use super::*;

impl<'a, S> VdLeanTranspilationBuilder<'a, S>
where
    S: IsVdLeanTranspilationScheme,
{
    pub(super) fn build_old_main_hypothesis_tactics(
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
