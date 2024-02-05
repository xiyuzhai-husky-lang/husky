use super::*;
use either::*;
use husky_eth_signature::{FugitiveEthTemplate, HasEthTemplate, TypeVariantEthTemplate};
use husky_eth_term::instantiation::EthInstantiate;
use husky_fly_term::instantiation::{
    FlyInstantiate, FlyInstantiation, FlyInstantiationEnvironment, FlyTermSymbolResolution,
};
use husky_regional_token::IdentRegionalToken;
use maybe_result::*;

impl<'a> SemaExprEngine<'a> {
    /// only returns None for Option<FlyInstantiation> if this is an ontology constructor
    pub(super) fn calc_principal_item_path_expr_ty(
        &mut self,
        syn_expr_idx: SynExprIdx,
        path: PrincipalEntityPath,
        expr_ty_expectation: &impl ExpectFlyTerm,
        ty_path_disambiguation: TypePathDisambiguation,
    ) -> (
        SemaExprDataResult<Option<FlyInstantiation>>,
        SemaExprTypeResult<FlyTerm>,
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
                            (
                                Ok(Some(instantiation)),
                                tmpl.instance_constructor_ty(db)
                                    .ok_or(OriginalSemaExprTypeError::NoConstructor.into())
                                    .map(Into::into),
                            )
                        }
                        Err(_) => todo!(),
                    },
                },
                MajorItemPath::Trait(path) => {
                    (Ok(None), path.ty(db).map(Into::into).map_err(Into::into))
                }
                MajorItemPath::Fugitive(path) => match path.eth_template(db) {
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
                            FugitiveEthTemplate::FunctionFn(tmpl) => FlyInstantiate::instantiate(
                                tmpl.ritchie_ty(db),
                                self,
                                syn_expr_idx,
                                &instantiation,
                            ),
                            FugitiveEthTemplate::FunctionGn(tmpl) => FlyInstantiate::instantiate(
                                tmpl.ritchie_ty(db),
                                self,
                                syn_expr_idx,
                                &instantiation,
                            ),
                            FugitiveEthTemplate::TypeAlias(_) => todo!(),
                            FugitiveEthTemplate::Val(tmpl) => FlyInstantiate::instantiate(
                                tmpl.return_ty(db),
                                self,
                                syn_expr_idx,
                                &instantiation,
                            )
                            .with_place(FlyPlace::Leashed),
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
    ) -> SemaExprTypeResult<FlyTerm> {
        let db = self.db();
        let parent_ty_path = path.parent_ty_path(db);
        match path.eth_template(db)? {
            TypeVariantEthTemplate::Props(_) => todo!(),
            TypeVariantEthTemplate::Unit(_) => match expr_ty_expectation.destination() {
                Some(destination) => match destination.data(self) {
                    FlyTermData::TypeOntology { ty_path, .. } if ty_path == parent_ty_path => {
                        Ok(destination)
                    }
                    _ => Ok(path.ty(db)?.into()),
                },
                None => Ok(path.ty(db)?.into()),
            },
            TypeVariantEthTemplate::Tuple(_) => Ok(path.ty(db)?.into()),
        }
    }
}
