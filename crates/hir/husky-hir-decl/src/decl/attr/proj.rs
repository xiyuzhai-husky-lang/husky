use super::*;
use husky_entity_path::path::major_item::MajorItemPath;
use husky_eth_term::term::EthTerm;
use husky_syn_decl::decl::attr::proj::ProjAttrSynDecl;
use husky_term_prelude::ItemPathTerm;

#[salsa::interned]
pub struct ProjAttrHirDecl {
    pub path: AttrItemPath,
    #[return_ref]
    pub projs: Vec<MajorItemPath>,
    pub hir_eager_expr_region: HirEagerExprRegion,
}
