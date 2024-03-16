mod builder;
pub mod destroyer;
pub mod eval;
pub mod expr;
pub mod jar;
pub mod pattern;
pub mod region;
pub mod stmt;
pub mod storage;
#[cfg(test)]
mod tests;
mod variable;
pub mod version_stamp;

use self::builder::VmirExprBuilder;
#[cfg(test)]
use self::tests::*;
use husky_task_interface::IsLinkageImpl;

pub(crate) trait ToVmir<LinkageImpl: IsLinkageImpl>: Copy {
    type Output;

    fn to_vmir(self, builder: &mut VmirExprBuilder<LinkageImpl>) -> Self::Output;
}
