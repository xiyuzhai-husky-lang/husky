use super::*;
use either::*;
use husky_entity_path::path::ty_variant::TypeVariantPath;
use husky_eth_signature::signature::{
    major_item::form::FormEthTemplate, ty_variant::TypeVariantEthTemplate, HasEthTemplate,
};
use husky_eth_term::instantiation::EthInstantiate;
use husky_fly_term::{
    instantiation::{
        FlyInstantiate, FlyInstantiation, FlyInstantiationEnvironment, FlyTermSymbolResolution,
    },
    quary::FlyQuary,
};
use husky_regional_token::IdentRegionalToken;
use maybe_result::*;

impl<'a> SemExprBuilder<'a> {
    /// only returns None for Option<FlyInstantiation> if this is an ontology constructor
    pub(super) fn calc_principal_item_path_expr_ty(
        &mut self,
        syn_expr_idx: SynExprIdx,
        path: PrincipalEntityPath,
        expr_ty_expectation: &impl ExpectFlyTerm,
        ty_path_disambiguation: TypePathDisambiguation,
    ) -> (
        SemExprDataResult<Option<FlyInstantiation>>,
        SemExprTypeResult<FlyTerm>,
    ) {
        let db = self.db();
        match path {
            PrincipalEntityPath::Module(_) => unreachable!(),
            PrincipalEntityPath::MajorItem(path) => match path {
                MajorItemPath::Type(path) => match ty_path_disambiguation {
                    // for ontology constructor, we don't need to fill in template parameters
                    TypePathDisambiguation::OntologyConstructor => (
                        Ok(None),
                        path.ty(db, ty_path_disambiguation)
                            .map(Into::into)
                            .map_err(Into::into),
                    ),
                    // for instance constructor, we need to fill in template parameters
                    TypePathDisambiguation::InstanceConstructor => match path.eth_template(db) {
                        Ok(tmpl) => {
                            let instantiation = FlyInstantiation::from_template_parameters(
                                path,
                                FlyInstantiationEnvironment::TypeOntologyConstructor,
                                syn_expr_idx,
                                tmpl.template_parameters(db),
                                None,
                                self.fly_terms_mut(),
                                db,
                            );
                            let instance_constructor_ty = tmpl.instance_constructor_ty(db);
                            (
                                Ok(Some(instantiation)),
                                instance_constructor_ty
                                    .ok_or(OriginalSemExprTypeError::NoConstructor { path }.into())
                                    .map(Into::into),
                            )
                        }
                        Err(_) => todo!(),
                    },
                },
                MajorItemPath::Trait(path) => {
                    (Ok(None), path.ty(db).map(Into::into).map_err(Into::into))
                }
                MajorItemPath::Form(path) => match path.eth_template(db) {
                    Ok(tmpl) => {
                        let instantiation = FlyInstantiation::from_template_parameters(
                            path,
                            FlyInstantiationEnvironment::TypeOntologyConstructor,
                            syn_expr_idx,
                            tmpl.template_parameters(db),
                            None,
                            self.fly_terms_mut(),
                            db,
                        );
                        let ty = match tmpl {
                            FormEthTemplate::Ritchie(tmpl) => FlyInstantiate::instantiate(
                                tmpl.ritchie_ty(db),
                                self,
                                syn_expr_idx,
                                &instantiation,
                            ),
                            FormEthTemplate::TypeAlias(_) => todo!(),
                            FormEthTemplate::Val(tmpl) => FlyInstantiate::instantiate(
                                tmpl.return_ty(db),
                                self,
                                syn_expr_idx,
                                &instantiation,
                            )
                            .with_quary(FlyQuary::Leashed { place_idx: None }),
                            FormEthTemplate::Static(_) => todo!(),
                            FormEthTemplate::Compterm(_) => todo!(),
                        };
                        (Ok(Some(instantiation)), Ok(ty))
                    }
                    Err(_) => todo!(),
                },
            },
            PrincipalEntityPath::TypeVariant(path) => {
                let parent_ty_path = path.parent_ty_path(db);
                let parent_ty_tmpl = match parent_ty_path.eth_template(db) {
                    Ok(tmpl) => tmpl,
                    Err(_) => todo!(),
                };
                let tmpl = match path.eth_template(db) {
                    Ok(tmpl) => tmpl,
                    Err(_) => todo!(),
                };
                let instantiation = FlyInstantiation::from_template_parameters(
                    path,
                    FlyInstantiationEnvironment::TypeOntologyConstructor,
                    syn_expr_idx,
                    parent_ty_tmpl.template_parameters(db),
                    None, // tmpl.template_parameters(db),
                    self.fly_terms_mut(),
                    db,
                );
                let ty = FlyInstantiate::instantiate(
                    tmpl.instance_constructor_ty(db),
                    self,
                    syn_expr_idx,
                    &instantiation,
                );
                (Ok(Some(instantiation)), Ok(ty))
            }
        }
    }

    fn calc_ty_variant_path_expr_ty(
        &mut self,
        path: TypeVariantPath,
        expr_ty_expectation: &impl ExpectFlyTerm,
    ) -> SemExprTypeResult<FlyTerm> {
        let db = self.db();
        let parent_ty_path = path.parent_ty_path(db);
        match path.eth_template(db)? {
            TypeVariantEthTemplate::Props(_) => todo!(),
            TypeVariantEthTemplate::Unit(_) => match expr_ty_expectation.destination() {
                FlyTermDestination::AnyOriginal | FlyTermDestination::AnyDerived => {
                    Ok(path.ty(db)?.into())
                }
                FlyTermDestination::Specific(destination) => match destination.data(self) {
                    FlyTermData::TypeOntology { ty_path, .. } if ty_path == parent_ty_path => {
                        Ok(destination)
                    }
                    _ => Ok(path.ty(db)?.into()),
                },
            },
            TypeVariantEthTemplate::Tuple(_) => Ok(path.ty(db)?.into()),
        }
    }
}
