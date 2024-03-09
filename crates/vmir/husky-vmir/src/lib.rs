pub mod block;
mod builder;
pub mod destroyer;
pub mod expr;
pub mod jar;
pub mod region;
pub mod stmt;
pub mod storage;
mod variable;
pub mod vmir;

use self::jar::VmirJar as Jar;
use builder::VmirExprBuilder;

pub(crate) trait ToVmir: Copy {
    type Output;

    fn to_vmir(self, builder: &mut VmirExprBuilder) -> Self::Output;
}
