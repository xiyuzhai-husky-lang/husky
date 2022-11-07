use crate::*;
use upcast::Upcast;

#[salsa::query_group(TermPatternInferDbStorage)]
pub trait TermPatternInferDb: TermDb + Upcast<dyn TermDb> {
    fn term_pattern_infer_sheet(&self, file: FileItd) -> FileResultArc<TermPatternInferSheet>;
}

fn term_pattern_infer_sheet(
    db: &dyn TermPatternInferDb,
    file: FileItd,
) -> FileResultArc<TermPatternInferSheet> {
    todo!()
}
