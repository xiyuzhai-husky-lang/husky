#![feature(trait_upcasting)]
#![feature(const_trait_impl)]
pub mod db;
mod error;
mod field;
mod method;
mod principal_item_path;
mod term;
#[cfg(test)]
mod tests;
mod variance;

pub use self::db::*;
pub use self::error::*;
pub use self::field::*;
pub use self::method::*;
pub use self::principal_item_path::*;
pub use self::term::*;

#[cfg(test)]
use self::tests::*;
use self::variance::*;
use husky_coword::*;
use husky_declarative_signature::*;
use husky_declarative_term::*;
use husky_entity_path::*;

use husky_term_prelude::*;

pub trait HasDeclarativeType: Copy {
    fn declarative_ty(self, db: &::salsa::Db) -> DeclarativeTypeResult<DeclarativeTerm>;
}
