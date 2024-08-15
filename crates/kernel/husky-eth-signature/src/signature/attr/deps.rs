use super::*;
use husky_dec_signature::signature::attr::dep::{DepAttrDecTemplate, DepAttrShardDecTemplate};
use husky_term_prelude::TypeFinalDestinationExpectation;

#[salsa::interned]
pub struct DepAttrEthTemplate {
    pub path: AttrItemPath,
    pub shards: SmallVec<[DepAttrShardEthTemplate; 8]>,
}

#[salsa::interned]
pub struct DepAttrShardEthTemplate {
    pub dep_term: EthTerm,
}

impl DepAttrEthTemplate {
    pub(super) fn from_dec(
        path: AttrItemPath,
        dec_template: DepAttrDecTemplate,
        db: &::salsa::Db,
    ) -> EthSignatureResult<Self> {
        let shards = dec_template
            .shards(db)
            .iter()
            .map(|&shard| DepAttrShardEthTemplate::from_dec(shard, db))
            .collect::<EthSignatureResult<_>>()?;
        Ok(Self::new(db, path, shards))
    }
}

impl DepAttrShardEthTemplate {
    fn from_dec(
        dec_template: DepAttrShardDecTemplate,
        db: &::salsa::Db,
    ) -> EthSignatureResult<Self> {
        Ok(Self::new(
            db,
            EthTerm::from_dec(
                db,
                dec_template.dep_term(db),
                TypeFinalDestinationExpectation::Any,
            )?,
        ))
    }
}
