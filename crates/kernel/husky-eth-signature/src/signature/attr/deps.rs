use super::*;
use husky_dec_signature::signature::attr::deps::{DepsAttrDecTemplate, DepsAttrShardDecTemplate};
use husky_term_prelude::TypeFinalDestinationExpectation;

#[salsa::interned]
pub struct DepsAttrEthTemplate {
    pub path: AttrItemPath,
    pub shards: SmallVec<[DepsAttrShardEthTemplate; 8]>,
}

#[salsa::interned]
pub struct DepsAttrShardEthTemplate {
    pub dep_term: EthTerm,
}

impl DepsAttrEthTemplate {
    pub(super) fn from_dec(
        path: AttrItemPath,
        dec_template: DepsAttrDecTemplate,
        db: &::salsa::Db,
    ) -> EthSignatureResult<Self> {
        let shards = dec_template
            .shards(db)
            .iter()
            .map(|&shard| DepsAttrShardEthTemplate::from_dec(shard, db))
            .collect::<EthSignatureResult<_>>()?;
        Ok(Self::new(db, path, shards))
    }
}

impl DepsAttrShardEthTemplate {
    fn from_dec(
        dec_template: DepsAttrShardDecTemplate,
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
