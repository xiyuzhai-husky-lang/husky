mod absolute;
mod alias;
mod db;
mod error;
mod implementation;
#[cfg(test)]
mod tests;

pub use absolute::*;
pub use alias::*;
pub use db::EntityTreeDb;

use error::EntityTreeError;
use husky_ast::AstIdx;
use husky_entity_path::EntityPath;
use idx_arena::{Arena, ArenaIdxRange};
use implementation::ImplementationMap;
#[cfg(test)]
use tests::*;

#[salsa::jar(db = EntityTreeDb)]
pub struct EntityTreeJar(absolute_entity_path);

pub struct EntityTreeSheet {}

pub struct EntityNodeSheet {
    arena: Arena<EntityNode>,
    implementations: ImplementationMap,
    errors: Vec<EntityTreeError>,
}

pub type EntityTreeNodeIdxRange = ArenaIdxRange<EntityNode>;

pub struct EntityNode {
    entity_path: EntityPath,
    accessibility: Accessibility,
    kind: EntityNodeKind,
    children: Option<EntityTreeNodeIdxRange>,
}

enum Accessibility {
    PubCrate,              // this is default
    Public,                // everyone can access it
    Protected(EntityPath), // only one module and its submodules
    Private,               // only self
}

enum EntityNodeKind {
    Alias(EntityPath),
    Type,
}
