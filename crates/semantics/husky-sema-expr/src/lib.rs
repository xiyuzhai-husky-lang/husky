#![feature(let_chains)]
#![feature(if_let_guard)]
#![feature(trait_upcasting)]
#![feature(const_trait_impl)]
#![allow(unused, warnings)]
mod db;
mod engine;
mod error;
mod expr;
// mod info;
mod obelisks;
mod region;
mod stmt;
#[cfg(test)]
mod tests;

pub use self::db::*;
pub use self::engine::*;
pub use self::error::*;
pub use self::expr::*;
pub use self::obelisks::*;
pub use self::stmt::*;
// pub use self::info::*;
pub use self::region::*;

use either::*;
use husky_declarative_signature::*;
use husky_entity_kind::*;
use husky_entity_path::*;
use husky_entity_syn_tree::*;
use husky_ethereal_term::*;
use husky_fluffy_term::*;
use husky_syn_defn::*;
use husky_syn_expr::*;
use husky_term_prelude::*;
use salsa::DebugWithDb;
use smallvec::*;

#[cfg(test)]
use tests::*;

#[salsa::jar(db = SemaExprDb)]
pub struct SemaExprJar(
    ty_ontology_path_unveil_impl_block_signature_templates,
    ty_ontology_application_unveil_impl_block_signature_templates,
    sema_expr_region,
);
