use super::*;
use husky_syn_decl::decl::attr::deps::DepsAttrSynDecl;

#[salsa::interned]
pub struct DepsAttrDecTemplate {
    pub path: AttrItemPath,
    pub shards: SmallVec<[DepsAttrShardDecTemplate; 8]>,
}

#[salsa::interned]
pub struct DepsAttrShardDecTemplate {
    pub dep_term: DecTerm,
}

impl DepsAttrDecTemplate {
    pub(super) fn from_decl(
        path: AttrItemPath,
        decl: DepsAttrSynDecl,
        db: &::salsa::Db,
    ) -> DecSignatureResult<Self> {
        let syn_expr_region = decl.syn_expr_region(db);
        let dec_term_region = syn_expr_dec_term_region(db, syn_expr_region);
        let shards = decl
            .deps(db)
            .iter()
            .map(|dep| {
                Ok(DepsAttrShardDecTemplate::new(
                    db,
                    dec_term_region.expr_term(dep.syn_expr_idx())?,
                ))
            })
            .collect::<SynExprDecTermResultRef<_>>()?;
        Ok(Self::new(db, path, shards))
    }
}
