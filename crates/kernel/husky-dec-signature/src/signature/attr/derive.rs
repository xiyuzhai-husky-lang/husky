use super::*;

#[salsa::interned]
pub struct DeriveAttrDecTemplate {
    pub shards: SmallVec<[DeriveAttrShardDecTemplate; 8]>,
}

#[salsa::interned]
pub struct DeriveAttrShardDecTemplate {
    pub trai_term: DecTerm,
}

impl DeriveAttrDecTemplate {
    pub(super) fn from_decl(decl: DeriveAttrSynDecl, db: &::salsa::Db) -> DecSignatureResult<Self> {
        let syn_expr_region = decl.syn_expr_region(db);
        let dec_term_region = syn_expr_dec_term_region(db, syn_expr_region);
        let shards = decl
            .trais(db)
            .iter()
            .map(|trai| {
                Ok(DeriveAttrShardDecTemplate::new(
                    db,
                    dec_term_region.expr_term(trai.syn_expr_idx())?,
                ))
            })
            .collect::<SynExprDecTermResultRef<_>>()?;
        Ok(DeriveAttrDecTemplate::new(db, shards))
    }
}
