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
