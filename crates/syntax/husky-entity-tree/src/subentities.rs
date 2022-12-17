use crate::{node::EntityNode, *};
use husky_ast::AstIdx;
use husky_entity_path::EntityPath;
use husky_vfs::VfsResult;

pub(crate) fn subentities() -> Vec<EntityNode> {
    todo!()
}

#[salsa::tracked(jar = EntityTreeJar, return_ref)]
pub(crate) fn submodules(
    db: &dyn EntityTreeDb,
    module: EntityPath,
) -> VfsResult<Vec<SubmoduleDecl>> {
    todo!()
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubmoduleDecl {
    pub ast_idx: AstIdx,
}
