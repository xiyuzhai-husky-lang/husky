use super::*;

#[salsa::tracked]
pub struct TaskCrateSynNodeDecl {
    #[id]
    pub path: CratePath,
    pub syn_expr_region: SynExprRegion,
}

impl<'db> CrateDeclParser<'db> {
    pub(super) fn parse_task_crate_syn_node_decl(self) -> TaskCrateSynNodeDecl {
        todo!()
    }
}

#[salsa::tracked]
pub struct TaskCrateSynDecl {
    #[id]
    pub path: CratePath,
    pub syn_expr_region: SynExprRegion,
}

impl TaskCrateSynDecl {
    pub(super) fn from_node(
        path: CratePath,
        syn_node_decl: TaskCrateSynNodeDecl,
        db: &::salsa::Db,
    ) -> SynDeclResult<Self> {
        let syn_expr_region = syn_node_decl.syn_expr_region(db);
        Ok(Self::new(db, path, syn_expr_region))
    }
}
