pub mod builder;
mod graph_dynamics;
mod jar;
pub mod region;
pub mod static_mut_deps;
#[cfg(test)]
mod tests;

use self::jar::SemStaticMutDepsJar as Jar;
#[cfg(test)]
use self::tests::*;
