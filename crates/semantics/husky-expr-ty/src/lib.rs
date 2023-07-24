#![feature(if_let_guard)]
#![feature(trait_upcasting)]
#![feature(const_trait_impl)]
mod db;
mod engine;
mod error;
mod info;
mod region;
#[cfg(test)]
mod tests;

pub use self::db::*;
pub use self::error::*;
pub use self::info::*;
pub use self::region::*;

use self::engine::*;
use either::*;
use husky_declarative_signature::*;
use husky_ethereal_term::*;
use husky_fluffy_term::*;
use husky_item_path::*;
use husky_item_taxonomy::*;
use husky_item_tree::*;
use husky_syn_defn::*;
use husky_syn_expr::*;
use husky_term_prelude::*;
use salsa::DebugWithDb;
use smallvec::*;

#[cfg(test)]
use tests::*;

#[salsa::jar(db = ExprTypeDb)]
pub struct ExprTypeJar(
    ty_ontology_path_unveil_impl_block_signature_templates,
    ty_ontology_application_unveil_impl_block_signature_templates,
    expr_ty_region,
);
