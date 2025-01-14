use visored_mir_expr::expr::{application::VdMirFunc, VdMirExprIdx};
use visored_signature::signature::separator::base::VdBaseSeparatorSignature;

use super::*;

impl<'a, S> VdLeanTranspilationBuilder<'a, S>
where
    S: IsVdLeanTranspilationScheme,
{
    pub(super) fn build_nontrivial_chain_hypothesis_tactics(
        &mut self,
        hypothesis_entry: &VdMirHypothesisEntry,
        leader: VdMirExprIdx,
        followers: &[(VdMirFunc, VdMirExprIdx)],
        joined_signature: VdBaseSeparatorSignature,
        ln_tactics: &mut Vec<LnMirTacticData>,
    ) {
        match hypothesis_entry.construction() {
            VdMirHypothesisConstruction::Sorry => (),
            VdMirHypothesisConstruction::Kurapika => todo!(),
            _ => unreachable!(),
        }
    }
}
