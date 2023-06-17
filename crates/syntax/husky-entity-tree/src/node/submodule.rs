use super::*;

#[salsa::tracked(db = EntityTreeDb, jar = EntityTreeJar)]
pub struct SubmoduleNode {
    #[id]
    pub path: ModulePath,
    pub visibility: Scope,
    pub ast_idx: AstIdx,
    pub ident_token: IdentToken,
}

// todo: finish this
struct SubmoduleNodePath;

impl SubmoduleNodePath {
    fn node(self, db: &dyn EntityTreeDb) -> SubmoduleNode {
        todo!()
    }
}
