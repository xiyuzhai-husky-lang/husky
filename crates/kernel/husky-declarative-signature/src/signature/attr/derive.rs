use super::*;

#[salsa::interned(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
pub struct DeriveAttrDeclarativeSignatureTemplate {
    pub shards: SmallVec<[DeriveAttrShardDeclarativeSignatureTemplate; 8]>,
}

#[salsa::interned(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
pub struct DeriveAttrShardDeclarativeSignatureTemplate {
    pub trai_term: DeclarativeTerm,
}

impl DeriveAttrDeclarativeSignatureTemplate {
    pub(super) fn from_decl(
        decl: DeriveAttrSynDecl,
        db: &::salsa::Db,
    ) -> DeclarativeSignatureResult<Self> {
        let syn_expr_region = decl.syn_expr_region(db);
        let declarative_term_region = declarative_term_region(db, syn_expr_region);
        let shards = decl
            .trais(db)
            .iter()
            .map(|trai| {
                Ok(DeriveAttrShardDeclarativeSignatureTemplate::new(
                    db,
                    declarative_term_region.expr_term(trai.syn_expr_idx())?,
                ))
            })
            .collect::<DeclarativeTermResultBorrowed2<_>>()?;
        Ok(DeriveAttrDeclarativeSignatureTemplate::new(db, shards))
    }
}
