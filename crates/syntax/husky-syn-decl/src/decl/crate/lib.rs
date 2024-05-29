use husky_crate_decl_ast::CrateDeclAst;

use super::*;

#[salsa::tracked]
pub struct LibCrateSynNodeDecl {
    #[id]
    pub path: CratePath,
    pub syn_expr_region: SynExprRegion,
}

impl<'db> CrateDeclParser<'db> {
    pub(super) fn parse_lib_crate_syn_node_decl(self) -> LibCrateSynNodeDecl {
        let db = self.db();
        let path = self.crate_path();
        let crate_decl_tokra_region_data = self.crate_decl_tokra_region_data();
        for ast_idx in crate_decl_tokra_region_data.root_body() {
            match crate_decl_tokra_region_data[ast_idx] {
                CrateDeclAst::Err => todo!(),
                CrateDeclAst::BasicStmtOrBranch {
                    regional_token_verse_idx,
                    body,
                } => todo!(),
                CrateDeclAst::IfElseStmts {
                    if_branch,
                    elif_branches,
                    else_branch,
                } => todo!(),
                CrateDeclAst::MatchStmt {
                    regional_token_verse_idx,
                    pattern_stmt,
                    case_branches,
                } => todo!(),
            }
        }
        let syn_expr_region = self.finish();
        LibCrateSynNodeDecl::new(db, path, syn_expr_region)
    }
}
