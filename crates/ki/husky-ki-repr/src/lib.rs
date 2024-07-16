#![feature(impl_trait_in_assoc_type)]
pub mod expansion;
pub mod jar;
pub mod repr;
#[cfg(test)]
mod tests;
pub mod var_deps;

use self::expansion::*;
use self::jar::KiReprJar as Jar;
use self::repr::*;
#[cfg(test)]
use self::tests::*;
