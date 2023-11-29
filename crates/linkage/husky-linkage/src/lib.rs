#![feature(trait_upcasting)]
#![feature(associated_type_bounds)]
pub mod dependency;
pub mod form;
pub mod instantiation;
pub mod jar;
pub mod linkage;
mod pantheon;
pub mod place;
pub mod root;
pub mod template_argument;
#[cfg(test)]
mod tests;
pub mod trai;
pub mod trai_for_ty;
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
