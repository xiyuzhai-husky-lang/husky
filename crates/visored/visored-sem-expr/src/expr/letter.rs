use super::*;

#[derive(Debug, PartialEq, Eq)]
pub enum VdSemLetterDispatch {}

impl<'a> VdSemExprBuilder<'a> {
    pub(super) fn build_letter(
        &mut self,
        syn_expr: VdSynExprIdx,
        token_idx_range: LxTokenIdxRange,
        letter: LxMathLetter,
    ) -> VdSemExprData {
        let resolution = self.symbol_resolution_table();
        VdSemExprData::Letter {
            token_idx_range,
            letter,
            dispatch: todo!(),
        }
    }
}
