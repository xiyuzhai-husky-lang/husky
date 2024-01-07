mod defn;
#[cfg(test)]
mod tests;

pub use self::defn::*;

use husky_entity_path::*;
use husky_entity_syn_tree::*;
use husky_syn_decl::*;
use husky_syn_expr::*;
use husky_vfs::{ModulePath};

#[salsa::jar(db = SynDefnDb)]
pub struct SynDefnJar(crate::defn::item_syn_node_defn);
