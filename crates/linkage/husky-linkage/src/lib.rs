#![feature(trait_upcasting)]
#![feature(associated_type_bounds)]
/// the name amazon comes from diablo 2
pub mod amazon;
pub mod dependency;
pub mod form;
pub mod instantiation;
pub mod jar;
pub mod linkage;
pub mod pantheon;
pub mod path;
pub mod place;
pub mod template_argument;
#[cfg(test)]
mod tests;
pub mod trai;
pub mod trai_for_ty;
/// the name valkyrie comes from diablo 2
pub mod valkyrie;
pub mod version_stamp;

use self::instantiation::*;
use self::jar::*;
use self::linkage::*;
use self::place::*;
use self::template_argument::*;
#[cfg(test)]
use self::tests::*;
use self::version_stamp::*;
use ::version_stamp::HasVersionStamp;
use husky_entity_path::ItemPath;
