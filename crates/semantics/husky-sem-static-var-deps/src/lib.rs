#![feature(generic_const_exprs)]
mod builder;
mod graph_dynamics;
pub mod jar;
pub mod region;
pub mod static_var_deps;
#[cfg(test)]
mod tests;

use self::jar::SemStaticVarDepsJar as Jar;
#[cfg(test)]
use self::tests::*;
