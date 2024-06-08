#![warn(type_alias_bounds)]
#![feature(generic_const_exprs)]
pub mod context;
pub mod cycle_group;
mod full_deps_cropped;
#[cfg(test)]
mod tests;

#[cfg(test)]
use self::tests::Jar;
