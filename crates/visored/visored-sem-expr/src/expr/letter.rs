use super::*;
use symbol::local_defn::VdSemSymbolLocalDefnIdx;
use visored_global_resolution::resolution::letter::VdLetterGlobalResolution;
use visored_syn_expr::symbol::resolution::{
    letter::VdSynLetterSymbolResolution, VdSynSymbolResolution,
};

#[derive(Debug, PartialEq, Eq)]
pub enum VdSemLetterDispatch {
    Global(VdLetterGlobalResolution),
    Local(VdSemSymbolLocalDefnIdx),
}

impl<'a> VdSemExprBuilder<'a> {
    pub(super) fn build_letter(
        &mut self,
        syn_expr: VdSynExprIdx,
        token_idx_range: LxTokenIdxRange,
        letter: LxMathLetter,
    ) -> (VdSemExprData, VdZfcType) {
        let resolution = &self.syn_symbol_resolution_table()[syn_expr];
        let dispatch = match resolution {
            Ok(resolution) => {
                let VdSynSymbolResolution::Letter(resolution) = resolution else {
                    unreachable!()
                };
                self.build_letter_dispatch(resolution)
            }
            Err(e) => todo!("letter = `{letter}`, e = {e}"),
        };
        let ty = match dispatch {
            VdSemLetterDispatch::Global(global_resolution) => {
                self.infer_letter_ty_from_global_resolution(global_resolution)
            }
            VdSemLetterDispatch::Local(local_defn) => {
                self.infer_letter_ty_from_local_defn(local_defn)
            }
        };
        (
            VdSemExprData::Letter {
                token_idx_range,
                letter,
                dispatch,
            },
            ty,
        )
    }

    fn build_letter_dispatch(
        &mut self,
        resolution: &VdSynLetterSymbolResolution,
    ) -> VdSemLetterDispatch {
        match *resolution {
            VdSynLetterSymbolResolution::Global(global_resolution) => {
                VdSemLetterDispatch::Global(global_resolution)
            }
            VdSynLetterSymbolResolution::Local(local_defn_idx) => {
                VdSemLetterDispatch::Local(local_defn_idx.to_vd_sem(self))
            }
        }
    }

    fn infer_letter_ty_from_global_resolution(
        &mut self,
        global_resolution: VdLetterGlobalResolution,
    ) -> VdZfcType {
        match global_resolution {
            VdLetterGlobalResolution::Item(item_path) => todo!(),
        }
    }

    fn infer_letter_ty_from_local_defn(
        &mut self,
        local_defn: VdSemSymbolLocalDefnIdx,
    ) -> VdZfcType {
        // self.symbol_local_defn_storage()[local_defn].ty()
        todo!()
    }
}
