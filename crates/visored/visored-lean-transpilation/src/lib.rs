mod builder;
pub mod dictionary;
mod expr;
pub mod helpers;
pub mod mangle;
pub mod namespace;
pub mod scheme;
pub mod stmt;
pub mod tactic;
#[cfg(test)]
mod tests;
pub mod ty;

use self::builder::VdLeanTranspilationBuilder;
use self::scheme::IsVdLeanTranspilationScheme;
#[cfg(test)]
use self::tests::*;
use visored_models::VdModels;

pub trait VdTranspileToLean<S, T>: Copy
where
    S: IsVdLeanTranspilationScheme,
{
    fn to_lean(self, builder: &mut VdLeanTranspilationBuilder<S>) -> T;
}
