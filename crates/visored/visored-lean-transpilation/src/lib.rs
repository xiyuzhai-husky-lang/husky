mod builder;
pub mod jar;
mod sem;
#[cfg(test)]
mod tests;

use self::jar::VdLeanTranspilationJar as Jar;
use crate::builder::VdLeanTranspilationBuilder;

pub trait ToLean: Copy {
    type Target;

    fn to_lean(self, builder: &mut VdLeanTranspilationBuilder) -> Self::Target;
}
