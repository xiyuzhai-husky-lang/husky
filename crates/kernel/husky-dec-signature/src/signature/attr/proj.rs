use super::*;
use husky_syn_decl::decl::attr::proj::ProjAttrSynDecl;

#[salsa::interned]
pub struct ProjAttrDecTemplate {
    pub path: AttrItemPath,
    pub shards: SmallVec<[ProjAttrShardDecTemplate; 8]>,
}

#[salsa::interned]
pub struct ProjAttrShardDecTemplate {
    pub proj_term: DecTerm,
}

impl ProjAttrDecTemplate {
    pub(super) fn from_decl(
        path: AttrItemPath,
        decl: ProjAttrSynDecl,
        db: &::salsa::Db,
    ) -> DecSignatureResult<Self> {
        let syn_expr_region = decl.syn_expr_region(db);
        let dec_term_region = syn_expr_dec_term_region(db, syn_expr_region);
        let shards = decl
            .projs(db)
            .iter()
            .map(|dep| {
                Ok(ProjAttrShardDecTemplate::new(
                    db,
                    dec_term_region.expr_term(dep.syn_expr_idx())?,
                ))
            })
            .collect::<SynExprDecTermResultRef<_>>()?;
        Ok(Self::new(db, path, shards))
    }
}
