mod builder;
pub mod clause;
pub mod division;
pub mod error;
pub mod expr;
pub mod helpers;
pub mod jar;
pub mod phrase;
pub mod range;
pub mod region;
pub mod sentence;
pub mod stmt;
#[cfg(feature = "test_helpers")]
pub mod test_helpers;
#[cfg(test)]
mod tests;

use self::jar::VdSemExprJar as Jar;
#[cfg(test)]
use self::tests::*;
use builder::VdSemExprBuilder;
use either::*;
use expr::VdSemExprIdx;
use stmt::VdSemStmtIdxRange;
use visored_syn_expr::{expr::VdSynExprIdx, stmt::VdSynStmtIdxRange};

pub trait ToVdSem<T> {
    fn to_vd_sem(self, builder: &mut VdSemExprBuilder) -> T;
}

impl<'db> ToVdSem<Either<VdSemExprIdx, VdSemStmtIdxRange>>
    for Either<VdSynExprIdx, VdSynStmtIdxRange>
{
    fn to_vd_sem(self, builder: &mut VdSemExprBuilder) -> Either<VdSemExprIdx, VdSemStmtIdxRange> {
        todo!()
        // match self {
        //     Left(syn_expr_idx) => Left(builder.expr_arena.insert(syn_expr_idx.into())),
        //     Right(syn_stmt_idx_range) => Right(syn_stmt_idx_range),
        // }
    }
}
