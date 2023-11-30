#![feature(trait_upcasting)]
#![feature(associated_type_bounds)]
pub mod jar;
pub mod linkage;
#[cfg(test)]
mod tests;
pub mod version_stamp;

use self::jar::*;
use self::linkage::*;
use self::place::*;
use self::template_argument::*;
#[cfg(test)]
use self::tests::*;
use self::version_stamp::*;
use ::version_stamp::HasVersionStamp;
use husky_entity_path::ItemPath;
