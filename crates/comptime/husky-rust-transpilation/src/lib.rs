//! this crate is an ad hoc implementation for husky to be able to compile
#![allow(warnings)]
#![feature(let_chains)]
#![feature(if_let_guard)]
mod binding;
mod builder;
mod defn;
mod expr;
mod fmt;
mod ingredient;
pub mod jar;
mod linkage;
mod manifest;
mod package;
mod path;
#[cfg(test)]
mod tests;
pub mod transpile_to_fs;
mod unit;

use self::builder::*;
use self::jar::*;
#[cfg(test)]
use self::tests::*;
use husky_vfs::{CratePath, ModulePath, PackagePath};
