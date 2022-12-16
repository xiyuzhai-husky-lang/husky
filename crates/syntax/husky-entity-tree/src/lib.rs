#![feature(trait_upcasting)]
mod absolute;
mod alias;
mod ast;
mod child;
mod db;
mod error;
mod implementation;
mod node;
#[cfg(test)]
mod tests;

pub use absolute::*;
pub use alias::*;
pub use db::EntityTreeDb;
pub use error::*;

use ast::*;
use error::EntityTreeError;
use husky_ast::AstIdx;
use husky_entity_kind::EntityKind;
use husky_entity_path::EntityPath;
use idx_arena::{Arena, ArenaIdxRange};
use implementation::ImplementationMap;
use node::*;
#[cfg(test)]
use tests::*;

#[salsa::jar(db = EntityTreeDb)]
pub struct EntityTreeJar(absolute_entity_path);

pub struct EntityTreeSheet {}
