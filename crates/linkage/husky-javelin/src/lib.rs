#![feature(associated_type_bounds)]
pub mod instantiation;
pub mod jar;
pub mod javelin;
pub mod path;
pub mod template_argument;
#[cfg(test)]
mod tests;
pub mod version_stamp;

use self::jar::JavelinJar;
use self::javelin::Javelin;
#[cfg(test)]
use self::tests::*;
