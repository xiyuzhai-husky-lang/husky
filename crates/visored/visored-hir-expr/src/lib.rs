mod builder;
pub mod expr;
pub mod jar;
pub mod stmt;
#[cfg(test)]
mod tests;

use self::builder::VdHirExprBuilder;
use self::jar::VdHirExprJar as Jar;

pub trait ToVdHir: Copy {
    type Output;

    fn to_hir(self, builder: &mut VdHirExprBuilder) -> Self::Output;
}
