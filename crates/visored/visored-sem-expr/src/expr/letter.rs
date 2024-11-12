use super::*;
use visored_syn_expr::symbol::resolution::{
    letter::VdSynLetterSymbolResolution, VdSynSymbolResolution,
};

#[derive(Debug, PartialEq, Eq)]
pub enum VdSemLetterDispatch {}

impl<'a> VdSemExprBuilder<'a> {
    pub(super) fn build_letter(
        &mut self,
        syn_expr: VdSynExprIdx,
        token_idx_range: LxTokenIdxRange,
        letter: LxMathLetter,
    ) -> VdSemExprData {
        let resolution = &self.symbol_resolution_table()[syn_expr];
        let dispatch = match resolution {
            Ok(resolution) => {
                let VdSynSymbolResolution::Letter(resolution) = resolution else {
                    unreachable!()
                };
                self.build_letter_dispatch(resolution)
            }
            Err(_) => todo!(),
        };
        VdSemExprData::Letter {
            token_idx_range,
            letter,
            dispatch: todo!(),
        }
    }

    fn build_letter_dispatch(
        &mut self,
        resolution: &VdSynLetterSymbolResolution,
    ) -> VdSemLetterDispatch {
        match resolution {
            VdSynLetterSymbolResolution::Global(vd_letter_global_resolution) => todo!(),
            VdSynLetterSymbolResolution::Local(arena_idx) => todo!(),
        }
    }
}
