use husky_entity_syn_tree::HasAttrPaths;
use husky_print_utils::p;
use husky_term_prelude::TermTypeExpectation;
use vec_like::{OrderedSmallVecSet, SmallVecPairMap, SmallVecSet, VecMapGetEntry};

use super::*;

#[salsa::interned(db = EtherealSignatureDb, jar = EtherealSignatureJar)]
pub struct DeriveAttrEtherealSignatureTemplate {
    pub shards: SmallVec<[DeriveAttrShardEtherealSignatureTemplate; 8]>,
}

#[salsa::interned(db = EtherealSignatureDb, jar = EtherealSignatureJar)]
pub struct DeriveAttrShardEtherealSignatureTemplate {
    pub trai_term: EtherealTerm,
}

impl DeriveAttrEtherealSignatureTemplate {
    pub(super) fn from_declarative(
        db: &dyn EtherealSignatureDb,
        declarative_template: DeriveAttrDeclarativeSignatureTemplate,
    ) -> EtherealSignatureResult<Self> {
        let trai_term = declarative_template
            .shards(db)
            .iter()
            .map(|&shard| DeriveAttrShardEtherealSignatureTemplate::from_declarative(shard, db))
            .collect::<EtherealSignatureResult<_>>()?;
        Ok(DeriveAttrEtherealSignatureTemplate::new(db, trai_term))
    }
}

impl DeriveAttrShardEtherealSignatureTemplate {
    fn from_declarative(
        declarative_template: DeriveAttrShardDeclarativeSignatureTemplate,
        db: &dyn EtherealSignatureDb,
    ) -> EtherealSignatureResult<Self> {
        Ok(Self::new(
            db,
            EtherealTerm::from_declarative(
                db,
                declarative_template.trai_term(db),
                TermTypeExpectation::Any,
            )?,
        ))
    }
}

pub trait HasDeriveAttrShardEtherealSignatureTemplates: Copy {
    fn derive_attr_shard_ethereal_signature_templates_map(
        self,
        db: &dyn EtherealSignatureDb,
    ) -> EtherealSignatureResult<
        &[(
            TraitPath,
            OrderedSmallVecSet<DeriveAttrShardEtherealSignatureTemplate, 1>,
        )],
    >;

    fn derive_attr_ethereal_signature_templates(
        self,
        db: &dyn EtherealSignatureDb,
        trai_path: TraitPath,
    ) -> EtherealSignatureResult<Option<&[DeriveAttrShardEtherealSignatureTemplate]>> {
        match self
            .derive_attr_shard_ethereal_signature_templates_map(db)?
            .get_entry(trai_path)
        {
            Some((_, ethereal_signature_templates)) => Ok(Some(ethereal_signature_templates)),
            None => Ok(None),
        }
    }
}

impl HasDeriveAttrShardEtherealSignatureTemplates for TypePath {
    fn derive_attr_shard_ethereal_signature_templates_map(
        self,
        db: &dyn EtherealSignatureDb,
    ) -> EtherealSignatureResult<
        &[(
            TraitPath,
            OrderedSmallVecSet<DeriveAttrShardEtherealSignatureTemplate, 1>,
        )],
    > {
        Ok(ty_path_derive_attr_ethereal_signature_templates_map(db, self).as_ref()?)
    }
}

// todo: change to ordered map and set
// todo: use trait HasEtherealSignatureTemplate?
#[salsa::tracked(jar = EtherealSignatureJar, return_ref)]
fn ty_path_derive_attr_ethereal_signature_templates_map(
    db: &dyn EtherealSignatureDb,
    ty_path: TypePath,
) -> EtherealSignatureResult<
    SmallVecPairMap<TraitPath, OrderedSmallVecSet<DeriveAttrShardEtherealSignatureTemplate, 1>, 8>,
> {
    let mut map: SmallVecPairMap<
        TraitPath,
        OrderedSmallVecSet<DeriveAttrShardEtherealSignatureTemplate, 1>,
        8,
    > = Default::default();
    for attr_path in ty_path.attr_paths(db) {
        let AttrEtherealSignatureTemplate::Derive(template) =
            attr_path.ethereal_signature_template(db)?
        else {
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
