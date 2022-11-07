use crate::*;
use husky_file::{FileItd, FileResultArc};

#[salsa::query_group(TermPatternInferDbStorage)]
pub trait TermPatternInferDb {
    fn term_pattern_infer_sheet(&self, file: FileItd) -> FileResultArc<TermPatternInferSheet>;
}

fn term_pattern_infer_sheet(
    db: &dyn TermPatternInferDb,
    file: FileItd,
) -> FileResultArc<TermPatternInferSheet> {
    todo!()
}
