use super::*;

// basically a wrapper type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::debug_with_db(db = EntitySynTreeDb)]
#[salsa::wrap_id(jar = EntitySynTreeJar)]
pub struct DecrSynNodePath {
    path: DecrPath,
}

#[salsa::tracked(db = EntitySynTreeDb, jar = EntitySynTreeJar, constructor = new_inner)]
pub struct DecrSynNode {
    #[id]
    pub syn_node_path: DecrSynNodePath,
    pub ast_idx: AstIdx,
}
