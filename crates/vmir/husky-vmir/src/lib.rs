pub mod block;
mod builder;
pub mod destroyer;
pub mod eval;
pub mod expr;
pub mod jar;
pub mod region;
pub mod stmt;
pub mod storage;
#[cfg(test)]
mod tests;
mod variable;
#[deprecated]
pub mod vmir;

use self::jar::VmirJar as Jar;
#[cfg(test)]
use self::tests::*;
use builder::VmirExprBuilder;

pub(crate) trait ToVmir: Copy {
    type Output;

    fn to_vmir(self, builder: &mut VmirExprBuilder) -> Self::Output;
}
