use husky_print_utils::p;
use husky_term_prelude::TermTypeExpectation;
use vec_like::{SmallVecPairMap, SmallVecSet, VecMapGetEntry};

use super::*;

#[salsa::interned(db = EtherealSignatureDb, jar = EtherealSignatureJar)]
pub struct DeriveDecrEtherealSignatureTemplate {
    trai_term: EtherealTerm,
}

impl DeriveDecrEtherealSignatureTemplate {
    fn from_declarative(
        db: &dyn EtherealSignatureDb,
        declarative_template: DeriveDecrDeclarativeSignatureTemplate,
    ) -> EtherealSignatureResult<Self> {
        let trai_term = EtherealTerm::from_declarative(
            db,
            declarative_template.trai_term(db),
            TermTypeExpectation::Any,
        )?;
        Ok(DeriveDecrEtherealSignatureTemplate::new(db, trai_term))
    }
}

pub trait HasDeriveDecrEtherealSignatureTemplates: Copy {
    fn derive_decr_ethereal_signature_templates_map(
        self,
        db: &dyn EtherealSignatureDb,
    ) -> EtherealSignatureResult<
        &[(
            TraitPath,
            EtherealSignatureResult<SmallVecSet<DeriveDecrEtherealSignatureTemplate, 1>>,
        )],
    >;

    fn derive_decr_ethereal_signature_templates(
        self,
        db: &dyn EtherealSignatureDb,
        trai_path: TraitPath,
    ) -> EtherealSignatureResult<Option<&[DeriveDecrEtherealSignatureTemplate]>> {
        match self
            .derive_decr_ethereal_signature_templates_map(db)?
            .get_entry(trai_path)
        {
            Some((_, ethereal_signature_templates)) => match ethereal_signature_templates {
                Ok(ethereal_signature_templates) => Ok(Some(ethereal_signature_templates)),
                Err(e) => Err(*e),
            },
            None => Ok(None),
        }
    }
}

impl HasDeriveDecrEtherealSignatureTemplates for TypePath {
    fn derive_decr_ethereal_signature_templates_map(
        self,
        db: &dyn EtherealSignatureDb,
    ) -> EtherealSignatureResult<
        &[(
            TraitPath,
            EtherealSignatureResult<SmallVecSet<DeriveDecrEtherealSignatureTemplate, 1>>,
        )],
    > {
        Ok(ty_path_derive_decr_ethereal_signature_templates_map(db, self).as_ref()?)
    }
}

// todo: change to ordered map and set
#[salsa::tracked(jar = EtherealSignatureJar, return_ref)]
fn ty_path_derive_decr_ethereal_signature_templates_map(
    db: &dyn EtherealSignatureDb,
    ty_path: TypePath,
) -> EtherealSignatureResult<
    SmallVecPairMap<
        TraitPath,
        EtherealSignatureResult<SmallVecSet<DeriveDecrEtherealSignatureTemplate, 1>>,
        8,
    >,
> {
    Ok(ty_path
        .derive_decr_declarative_signature_templates_map(db)?
        .iter()
        .map(
            |(trai_path, declarative_templates)| -> (
                TraitPath,
                EtherealSignatureResult<SmallVecSet<DeriveDecrEtherealSignatureTemplate, 1>>,
            ) {
                (
                    *trai_path,
                    match declarative_templates {
                        Ok(declarative_templates) => {
                            let mut ethereal_templates: SmallVecSet<
                                DeriveDecrEtherealSignatureTemplate,
                                1,
                            > = Default::default();
                            for declarative_template in declarative_templates.iter().copied() {
                                match DeriveDecrEtherealSignatureTemplate::from_declarative(
                                    db,
                                    declarative_template,
                                ) {
                                    Ok(ethereal_template) => {
                                        ethereal_templates.insert(ethereal_template)
                                    }
                                    Err(_) => todo!(),
                                }
                            }
                            Ok(ethereal_templates)
                        }
                        Err(e) => Err((*e).into()),
                    },
                )
            },
        )
        .collect())
}
