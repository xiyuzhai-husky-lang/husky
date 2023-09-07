use husky_entity_syn_tree::HasDecrPaths;
use husky_print_utils::p;
use husky_term_prelude::TermTypeExpectation;
use vec_like::{OrderedSmallVecSet, SmallVecPairMap, SmallVecSet, VecMapGetEntry};

use super::*;

#[salsa::interned(db = EtherealSignatureDb, jar = EtherealSignatureJar)]
pub struct DeriveDecrEtherealSignatureTemplate {
    pub shards: SmallVec<[DeriveDecrShardEtherealSignatureTemplate; 8]>,
}

#[salsa::interned(db = EtherealSignatureDb, jar = EtherealSignatureJar)]
pub struct DeriveDecrShardEtherealSignatureTemplate {
    pub trai_term: EtherealTerm,
}

impl DeriveDecrEtherealSignatureTemplate {
    pub(super) fn from_declarative(
        db: &dyn EtherealSignatureDb,
        declarative_template: DeriveDecrDeclarativeSignatureTemplate,
    ) -> EtherealSignatureResult<Self> {
        let trai_term = declarative_template
            .shards(db)
            .iter()
            .map(|&shard| DeriveDecrShardEtherealSignatureTemplate::from_declarative(shard, db))
            .collect::<EtherealSignatureResult<_>>()?;
        Ok(DeriveDecrEtherealSignatureTemplate::new(db, trai_term))
    }
}

impl DeriveDecrShardEtherealSignatureTemplate {
    fn from_declarative(
        declarative_template: DeriveDecrShardDeclarativeSignatureTemplate,
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

pub trait HasDeriveDecrShardEtherealSignatureTemplates: Copy {
    fn derive_decr_shard_ethereal_signature_templates_map(
        self,
        db: &dyn EtherealSignatureDb,
    ) -> EtherealSignatureResult<
        &[(
            TraitPath,
            OrderedSmallVecSet<DeriveDecrShardEtherealSignatureTemplate, 1>,
        )],
    >;

    fn derive_decr_ethereal_signature_templates(
        self,
        db: &dyn EtherealSignatureDb,
        trai_path: TraitPath,
    ) -> EtherealSignatureResult<Option<&[DeriveDecrShardEtherealSignatureTemplate]>> {
        match self
            .derive_decr_shard_ethereal_signature_templates_map(db)?
            .get_entry(trai_path)
        {
            Some((_, ethereal_signature_templates)) => Ok(Some(ethereal_signature_templates)),
            None => Ok(None),
        }
    }
}

impl HasDeriveDecrShardEtherealSignatureTemplates for TypePath {
    fn derive_decr_shard_ethereal_signature_templates_map(
        self,
        db: &dyn EtherealSignatureDb,
    ) -> EtherealSignatureResult<
        &[(
            TraitPath,
            OrderedSmallVecSet<DeriveDecrShardEtherealSignatureTemplate, 1>,
        )],
    > {
        Ok(ty_path_derive_decr_ethereal_signature_templates_map(db, self).as_ref()?)
    }
}

// todo: change to ordered map and set
// todo: use trait HasEtherealSignatureTemplate?
#[salsa::tracked(jar = EtherealSignatureJar, return_ref)]
fn ty_path_derive_decr_ethereal_signature_templates_map(
    db: &dyn EtherealSignatureDb,
    ty_path: TypePath,
) -> EtherealSignatureResult<
    SmallVecPairMap<TraitPath, OrderedSmallVecSet<DeriveDecrShardEtherealSignatureTemplate, 1>, 8>,
> {
    let mut map: SmallVecPairMap<
        TraitPath,
        OrderedSmallVecSet<DeriveDecrShardEtherealSignatureTemplate, 1>,
        8,
    > = Default::default();
    for decr_path in ty_path.decr_paths(db) {
        let DecrEtherealSignatureTemplate::Derive(template) =
            decr_path.ethereal_signature_template(db)?
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
