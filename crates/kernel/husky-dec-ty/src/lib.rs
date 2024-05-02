#![feature(const_trait_impl)]
mod error;
mod field;
pub mod jar;
mod method;
pub mod principal_item_path;
mod term;
#[cfg(test)]
mod tests;
pub mod variance;

pub use self::error::*;
pub use self::field::*;
pub use self::jar::*;
pub use self::method::*;
pub use self::term::*;

use self::principal_item_path::*;
#[cfg(test)]
use self::tests::*;
use self::variance::*;
use husky_coword::*;
use husky_dec_term::{jar::DecTermDb, term::*, *};
use husky_entity_path::*;
use husky_term_prelude::*;

pub trait HasDeclarativeType: Copy {
    fn declarative_ty(self, db: &::salsa::Db) -> DeclarativeTypeResult<DecTerm>;
}
