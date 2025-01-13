pub mod block;
mod builder;
pub mod clause;
pub mod division;
pub mod error;
pub mod expr;
pub mod helpers;
pub mod pattern;
pub mod phrase;
pub mod range;
pub mod sentence;
pub mod sheet;
pub mod symbol;
#[cfg(test)]
mod tests;

#[cfg(test)]
use self::tests::*;
use block::VdSemBlockIdxRange;
use builder::VdSemExprBuilder;
use either::*;
use expr::VdSemExprIdx;
use visored_models::VdModels;
use visored_syn_expr::{block::VdSynBlockIdxRange, expr::VdSynExprIdx};

pub trait ToVdSem<T> {
    fn to_vd_sem(self, builder: &mut VdSemExprBuilder) -> T;
}

impl<L, R, T, S> ToVdSem<Either<T, S>> for Either<L, R>
where
    L: ToVdSem<T>,
    R: ToVdSem<S>,
{
    fn to_vd_sem(self, builder: &mut VdSemExprBuilder) -> Either<T, S> {
        match self {
            Left(left) => Left(left.to_vd_sem(builder)),
            Right(right) => Right(right.to_vd_sem(builder)),
        }
    }
}
