use super::*;
use husky_dec_signature::signature::attr::proj::{ProjAttrDecTemplate, ProjAttrShardDecTemplate};
use husky_term_prelude::TypeFinalDestinationExpectation;

#[salsa::interned]
pub struct ProjAttrEthTemplate {
    pub path: AttrItemPath,
    pub shards: SmallVec<[ProjAttrShardEthTemplate; 8]>,
}

#[salsa::interned]
pub struct ProjAttrShardEthTemplate {
    pub dep_term: EthTerm,
}

impl ProjAttrEthTemplate {
    pub(super) fn from_dec(
        path: AttrItemPath,
        dec_template: ProjAttrDecTemplate,
        db: &::salsa::Db,
    ) -> EthSignatureResult<Self> {
        let shards = dec_template
            .shards(db)
            .iter()
            .map(|&shard| ProjAttrShardEthTemplate::from_dec(shard, db))
            .collect::<EthSignatureResult<_>>()?;
        Ok(Self::new(db, path, shards))
    }
}

impl ProjAttrShardEthTemplate {
    fn from_dec(
        dec_template: ProjAttrShardDecTemplate,
        db: &::salsa::Db,
    ) -> EthSignatureResult<Self> {
        Ok(Self::new(
            db,
            EthTerm::from_dec(
                db,
                dec_template.proj_term(db),
                TypeFinalDestinationExpectation::Any,
            )?,
        ))
    }
}
