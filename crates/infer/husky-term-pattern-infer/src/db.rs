use crate::*;
use husky_ast::AstDb;
use salsa::DbWithJar;
use upcast::Upcast;

pub trait TermPatternInferDb: DbWithJar<TermPatternInferJar> + TermDb + Upcast<dyn TermDb> {
    // fn term_pattern_infer_sheet(&self, file: AbsolutePath) -> VfsResult<TermPatternInferSheet>;
}

impl<T> TermPatternInferDb for T
where
    T: DbWithJar<TermPatternInferJar> + TermDb + Upcast<dyn TermDb>,
{
    // fn term_pattern_infer_sheet(&self, file: AbsolutePath) -> VfsResult<TermPatternInferSheet> {
    //     todo!()
    // }
}

// fn term_pattern_infer_sheet(
//     _db: &dyn TermPatternInferDb,
//     _file: AbsolutePath,
// ) -> VfsResult<TermPatternInferSheet> {
//     todo!()
// }
