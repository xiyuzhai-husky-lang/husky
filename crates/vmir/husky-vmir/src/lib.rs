mod builder;
pub mod destroyer;
pub mod eval;
pub mod expr;
pub mod jar;
pub mod region;
pub mod stmt;
#[cfg(test)]
mod tests;
mod variable;

use self::builder::VmirExprBuilder;
use self::jar::VmirJar as Jar;
#[cfg(test)]
use self::tests::*;

pub(crate) trait ToVmir: Copy {
    type Output;

    fn to_vmir(self, builder: &mut VmirExprBuilder) -> Self::Output;
}
