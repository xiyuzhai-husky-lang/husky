use super::*;

#[salsa::tracked(db = EntityTreeDb, jar = EntityTreeJar)]
pub struct SubmoduleNode {
    #[id]
    pub path: ModulePath,
    pub visibility: Scope,
    pub ast_idx: AstIdx,
    pub ident_token: IdentToken,
}
