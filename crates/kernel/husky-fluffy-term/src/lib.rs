#![feature(let_chains)]
#![feature(anonymous_lifetime_in_impl_trait)]
#![feature(generic_arg_infer)]
mod data;
mod db;
pub mod dispatch;
mod engine;
mod error;
mod expectation;
pub mod instantiation;
mod progress;
mod region;
mod resolve;
mod rewite;
pub mod signature;
mod term;
#[cfg(test)]
mod tests;
mod utils;

pub use self::data::*;
pub use self::db::*;
pub use self::dispatch::*;
pub use self::engine::*;
pub use self::error::*;
pub use self::expectation::*;
pub use self::progress::*;
pub use self::region::*;
pub use self::resolve::*;
pub use self::rewite::*;
pub use self::signature::*;
pub use self::term::*;

pub(crate) use self::signature::*;
#[cfg(test)]
pub(crate) use self::tests::*;

use self::instantiation::*;
use alt_option::*;
use either::*;
use husky_declarative_signature::*;
use husky_entity_path::*;
use husky_ethereal_term::*;
use husky_print_utils::p;
use husky_syn_expr::*;
use husky_term_prelude::*;
use maybe_result::*;
use salsa::DebugWithDb as _;
use smallvec::*;

#[salsa::jar(db = FluffyTermDb)]
pub struct FluffyTermJar(term_ritchie_fluffy_data, term_application_fluffy_data);
