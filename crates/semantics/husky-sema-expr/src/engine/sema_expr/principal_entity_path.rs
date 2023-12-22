use super::*;
use either::*;
use husky_ethereal_signature::{
    FugitiveEtherealSignatureTemplate, HasEtherealSignatureTemplate,
    TypeVariantEtherealSignatureTemplate,
};
use husky_ethereal_term::instantiation::EtherealInstantiate;
use husky_fluffy_term::instantiation::{
    FluffyInstantiate, FluffyInstantiation, FluffyInstantiationEnvironment,
    FluffyTermSymbolResolution,
};
use husky_regional_token::IdentRegionalToken;
use maybe_result::*;

impl<'a> SemaExprEngine<'a> {
    pub(super) fn calc_principal_item_path_expr_ty(
        &mut self,
        syn_expr_idx: SynExprIdx,
        path: PrincipalEntityPath,
        expr_ty_expectation: &impl ExpectFluffyTerm,
        ty_path_disambiguation: TypePathDisambiguation,
    ) -> (
        SemaExprDataResult<Option<FluffyInstantiation>>,
        SemaExprTypeResult<FluffyTerm>,
    ) {
        let db = self.db;
        match path {
            PrincipalEntityPath::Module(_) => unreachable!(),
            PrincipalEntityPath::MajorItem(path) => match path {
                MajorItemPath::Type(path) => match ty_path_disambiguation {
                    // for ontology constructor, we don't need to fill in template parameters
                    TypePathDisambiguation::OntologyConstructor => (
                        Ok(None),
                        path.ty(self.db, ty_path_disambiguation)
                            .map(Into::into)
                            .map_err(Into::into),
                    ),
                    // for instance constructor, we need to fill in template parameters
                    TypePathDisambiguation::InstanceConstructor => {
                        match path.ethereal_signature_template(db) {
                            Ok(tmpl) => {
                                let instantiation = FluffyInstantiation::from_template_parameters(
                                    FluffyInstantiationEnvironment::TypeOntologyConstructor,
                                    syn_expr_idx,
                                    tmpl.template_parameters(db),
                                    None,
                                    self.fluffy_terms_mut(),
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
                        }
                    }
                },
                MajorItemPath::Trait(path) => {
                    (Ok(None), path.ty(db).map(Into::into).map_err(Into::into))
                }
                MajorItemPath::Fugitive(path) => match path.ethereal_signature_template(db) {
                    Ok(tmpl) => {
                        let instantiation = FluffyInstantiation::from_template_parameters(
                            FluffyInstantiationEnvironment::TypeOntologyConstructor,
                            syn_expr_idx,
                            tmpl.template_parameters(db),
                            None,
                            self.fluffy_terms_mut(),
                            db,
                        );
                        let ty = match tmpl {
                            FugitiveEtherealSignatureTemplate::FunctionFn(tmpl) => {
                                FluffyInstantiate::instantiate(
                                    tmpl.ritchie_ty(db),
                                    self,
                                    syn_expr_idx,
                                    &instantiation,
                                )
                            }
                            FugitiveEtherealSignatureTemplate::FunctionGn(tmpl) => {
                                FluffyInstantiate::instantiate(
                                    tmpl.ritchie_ty(db),
                                    self,
                                    syn_expr_idx,
                                    &instantiation,
                                )
                            }
                            FugitiveEtherealSignatureTemplate::TypeAlias(_) => todo!(),
                            FugitiveEtherealSignatureTemplate::Val(tmpl) => {
                                FluffyInstantiate::instantiate(
                                    tmpl.return_ty(db),
                                    self,
                                    syn_expr_idx,
                                    &instantiation,
                                )
                            }
                        };
                        (Ok(Some(instantiation)), Ok(ty))
                    }
                    Err(_) => todo!(),
                },
            },
            PrincipalEntityPath::TypeVariant(path) => {
                let parent_ty_path = path.parent_ty_path(db);
                let parent_ty_tmpl = match parent_ty_path.ethereal_signature_template(db) {
                    Ok(tmpl) => tmpl,
                    Err(_) => todo!(),
                };
                let tmpl = match path.ethereal_signature_template(db) {
                    Ok(tmpl) => tmpl,
                    Err(_) => todo!(),
                };
                let instantiation = FluffyInstantiation::from_template_parameters(
                    FluffyInstantiationEnvironment::TypeOntologyConstructor,
                    syn_expr_idx,
                    parent_ty_tmpl.template_parameters(db),
                    None, // tmpl.template_parameters(db),
                    self.fluffy_terms_mut(),
                    db,
                );
                let ty = FluffyInstantiate::instantiate(
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
        expr_ty_expectation: &impl ExpectFluffyTerm,
    ) -> SemaExprTypeResult<FluffyTerm> {
        let parent_ty_path = path.parent_ty_path(self.db);
        match path.ethereal_signature_template(self.db)? {
            TypeVariantEtherealSignatureTemplate::Props(_) => todo!(),
            TypeVariantEtherealSignatureTemplate::Unit(_) => {
                match expr_ty_expectation.destination() {
                    Some(destination) => match destination.data(self) {
                        FluffyTermData::TypeOntology { ty_path, .. }
                            if ty_path == parent_ty_path =>
                        {
                            Ok(destination)
                        }
                        _ => Ok(path.ty(self.db)?.into()),
                    },
                    None => Ok(path.ty(self.db)?.into()),
                }
            }
            TypeVariantEtherealSignatureTemplate::Tuple(_) => Ok(path.ty(self.db)?.into()),
        }
    }
}
