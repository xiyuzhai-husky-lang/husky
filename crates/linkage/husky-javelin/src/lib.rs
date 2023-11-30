#![feature(associated_type_bounds)]
/// the name amazon comes from diablo 2
pub mod amazon;
pub mod instantiation;
pub mod jar;
pub mod javelin;
pub mod path;
pub mod template_argument;
#[cfg(test)]
mod tests;
/// the name valkyrie comes from diablo 2
pub mod valkyrie;
pub mod version_stamp;

use self::jar::JavelinJar;
use self::javelin::Javelin;
#[cfg(test)]
use self::tests::*;
