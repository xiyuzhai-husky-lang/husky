#![feature(trait_upcasting)]
#![feature(associated_type_bounds)]
pub mod instantiation;
pub mod jar;
pub mod linkage;
pub mod template_argument;
#[cfg(test)]
mod tests;
pub mod version_stamp;

use self::instantiation::*;
use self::jar::*;
use self::linkage::*;
#[cfg(test)]
use self::tests::*;
use self::version_stamp::*;
use ::version_stamp::HasVersionStamp;
use husky_entity_path::ItemPath;
