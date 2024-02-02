use husky_entity_tree::HasAttrPaths;

use husky_term_prelude::TermTypeExpectation;
use vec_like::{OrderedSmallVecSet, SmallVecPairMap, VecMapGetEntry};

use super::*;

#[salsa::interned(db = EtherealSignatureDb, jar = EtherealSignatureJar)]
pub struct DeriveAttrEthTemplate {
    pub shards: SmallVec<[DeriveAttrShardEthTemplate; 8]>,
}

#[salsa::interned(db = EtherealSignatureDb, jar = EtherealSignatureJar)]
pub struct DeriveAttrShardEthTemplate {
    pub trai_term: EthTerm,
}

impl DeriveAttrEthTemplate {
    pub(super) fn from_declarative(
        db: &::salsa::Db,
        declarative_template: DeriveAttrDecTemplate,
    ) -> EtherealSignatureResult<Self> {
        let trai_term = declarative_template
            .shards(db)
            .iter()
            .map(|&shard| DeriveAttrShardEthTemplate::from_declarative(shard, db))
            .collect::<EtherealSignatureResult<_>>()?;
        Ok(DeriveAttrEthTemplate::new(db, trai_term))
    }
}

impl DeriveAttrShardEthTemplate {
    fn from_declarative(
        declarative_template: DeriveAttrShardDecTemplate,
        db: &::salsa::Db,
    ) -> EtherealSignatureResult<Self> {
        Ok(Self::new(
            db,
            EthTerm::from_declarative(
                db,
                declarative_template.trai_term(db),
                TermTypeExpectation::Any,
            )?,
        ))
    }
}

pub trait HasDeriveAttrShardEthTemplates: Copy {
    fn derive_attr_shard_eth_templates_map(
        self,
        db: &::salsa::Db,
    ) -> EtherealSignatureResult<&[(TraitPath, OrderedSmallVecSet<DeriveAttrShardEthTemplate, 1>)]>;

    fn derive_attr_eth_templates(
        self,
        db: &::salsa::Db,
        trai_path: TraitPath,
    ) -> EtherealSignatureResult<Option<&[DeriveAttrShardEthTemplate]>> {
        match self
            .derive_attr_shard_eth_templates_map(db)?
            .get_entry(trai_path)
        {
            Some((_, eth_templates)) => Ok(Some(eth_templates)),
            None => Ok(None),
        }
    }
}

impl HasDeriveAttrShardEthTemplates for TypePath {
    fn derive_attr_shard_eth_templates_map(
        self,
        db: &::salsa::Db,
    ) -> EtherealSignatureResult<&[(TraitPath, OrderedSmallVecSet<DeriveAttrShardEthTemplate, 1>)]>
    {
        Ok(ty_path_derive_attr_eth_templates_map(db, self).as_ref()?)
    }
}

// todo: change to ordered map and set
// todo: use trait HasEthTemplate?
#[salsa::tracked(jar = EtherealSignatureJar, return_ref)]
fn ty_path_derive_attr_eth_templates_map(
    db: &::salsa::Db,
    ty_path: TypePath,
) -> EtherealSignatureResult<
    SmallVecPairMap<TraitPath, OrderedSmallVecSet<DeriveAttrShardEthTemplate, 1>, 8>,
> {
    let mut map: SmallVecPairMap<TraitPath, OrderedSmallVecSet<DeriveAttrShardEthTemplate, 1>, 8> =
        Default::default();
    for attr_path in ty_path.attr_paths(db) {
        let AttrEthTemplate::Derive(template) = attr_path.eth_template(db)? else {
            todo!()
        };
        for shard in template.shards(db) {
            let trai_path = shard.trai_term(db).leading_trai_path(db).expect("todo");
            map.update_value_or_insert(
                trai_path,
                |_| todo!(),
                OrderedSmallVecSet::new_one_elem_set(shard),
            )
        }
    }
    Ok(map)
}
