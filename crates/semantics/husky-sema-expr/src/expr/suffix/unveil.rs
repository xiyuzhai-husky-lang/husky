use super::*;
use husky_coword::coword_menu;
use husky_ethereal_signature::{
    helpers::trai_for_ty::*, EtherealSignatureError, EtherealSignatureMaybeResult,
    EtherealSignatureResult, TraitForTypeAssociatedTypeEtherealSignature,
    TraitForTypeImplBlockEtherealSignature, TraitForTypeImplBlockEtherealSignatureBuilder,
    TraitForTypeImplBlockEtherealSignatureTemplate,
};
use maybe_result::*;
use vec_like::VecMapGetEntry;

impl<'a> SemaExprEngine<'a> {
    pub(super) fn calc_unveil_expr_ty(
        &mut self,
        opd_syn_expr_idx: SynExprIdx,
        opr_regional_token_idx: RegionalTokenIdx,
    ) -> (
        SemaExprDataResult<SemaExprData>,
        SemaExprTypeResult<FluffyTerm>,
    ) {
        let db = self.db();
        self.unveiler.initialize_if_not(self.return_ty(), db);
        match self.unveiler {
            Unveiler::UniqueFullyInstantiated {
                opd_ty,
                unveil_output_ty,
                ref unveil_output_ty_signature,
                unveil_associated_fn_path,
                ..
            } => {
                let unveil_output_ty_signature = unveil_output_ty_signature.clone();
                let opd_sema_expr_idx = self.build_sema_expr(
                    opd_syn_expr_idx,
                    ExpectCoersion::new(TermContract::Move, opd_ty.into()),
                );
                (
                    Ok(SemaExprData::Unveil {
                        opd_sema_expr_idx,
                        opr_regional_token_idx,
                        unveil_output_ty_signature,
                        unveil_associated_fn_path,
                        return_ty: self.return_ty().unwrap(),
                    }),
                    Ok(unveil_output_ty.into()),
                )
            }
            Unveiler::UniquePartiallyInstanted { template } => {
                let (opd_sema_expr_idx, opd_ty) =
                    self.build_sema_expr_with_ty(opd_syn_expr_idx, ExpectAnyOriginal);
                let Some(opd_ty) = opd_ty else {
                    // p!(self.syn_expr_region_data.path().debug(db));
                    // p!(self.syn_expr_region_data[opd_syn_expr_idx].debug(db));
                    todo!()
                };
                let reduced_opd_ty: FluffyTerm = match opd_ty.base_ty_data(self) {
                    FluffyBaseTypeData::TypeOntology {
                        ty_path,
                        refined_ty_path: Left(PreludeTypePath::Indirection(indirection_path)),
                        ty_arguments,
                        ty_ethereal_term,
                    } => {
                        match indirection_path {
                            PreludeIndirectionTypePath::Ref => todo!(),
                            PreludeIndirectionTypePath::RefMut => todo!(),
                            PreludeIndirectionTypePath::Leash => {
                                // ad hoc
                                // needs to check more
                                ty_arguments[0]
                            }
                            PreludeIndirectionTypePath::At => todo!(),
                        }
                    }
                    _ => opd_ty,
                };
                match reduced_opd_ty.base_resolved(self) {
                    FluffyTermBase::Ethereal(opd_ty) => {
                        match template.instantiate_trai(&[opd_ty], db) {
                            JustOk(template) => {
                                let associated_output_template =
                                    match template.associated_output_template(db) {
                                        Ok(associated_output_template) => {
                                            associated_output_template
                                        }
                                        Err(e) => return (
                                            Err(DerivedSemaExprDataError::UnveilOutputTemplate {
                                                opd_sema_expr_idx,
                                                e,
                                            }
                                            .into()),
                                            Err(e.into()),
                                        ),
                                    };
                                let Some(unveil_output_ty_signature) =
                                    associated_output_template.try_into_signature(db)
                                else {
                                    todo!()
                                };
                                let ty_term = unveil_output_ty_signature.ty_term().into();
                                (
                                    Ok(SemaExprData::Unveil {
                                        opd_sema_expr_idx,
                                        opr_regional_token_idx,
                                        unveil_associated_fn_path: unveil_associated_fn_path(
                                            &unveil_output_ty_signature,
                                            db,
                                        ),
                                        unveil_output_ty_signature,
                                        return_ty: self.return_ty().unwrap(),
                                    }),
                                    Ok(ty_term),
                                )
                            }
                            JustErr(_) => todo!(),
                            Nothing => todo!(),
                        }
                    }
                    FluffyTermBase::Solid(_) => todo!(),
                    FluffyTermBase::Hollow(_) => todo!(),
                    FluffyTermBase::Place => todo!(),
                }
            }
            Unveiler::Nothing => (
                Err(todo!()),
                Err(OriginalSemaExprTypeError::CannotUnveil.into()),
            ),
            Unveiler::ErrUnableToInferReturnTypeForUnveiling => (
                Err(todo!()),
                Err(DerivedSemaExprTypeError::UnableToInferReturnTypeForUnveiling.into()),
            ),
            Unveiler::ErrEtherealSignature(e) => (Err(todo!()), Err(e.into())),
            Unveiler::Uninitialized => unreachable!(),
        }
    }

    pub(super) fn calc_unveil_expr_ty_given_opd_ty(
        &mut self,
        opd_ty: FluffyTerm,
    ) -> (
        SemaExprDataResult<SemaSuffixOpr>,
        SemaExprTypeResult<FluffyTerm>,
    ) {
        todo!()
    }
}

#[salsa::debug_with_db]
pub(crate) enum Unveiler {
    Uninitialized,
    UniqueFullyInstantiated {
        opd_ty: EtherealTerm,
        unveil_output_ty: EtherealTerm,
        unveil_output_ty_final_destination: FinalDestination,
        unveil_output_ty_signature: TraitForTypeAssociatedTypeEtherealSignature,
        unveil_associated_fn_path: TraitForTypeItemPath,
    },
    UniquePartiallyInstanted {
        template: TraitForTypeImplBlockEtherealSignatureBuilder,
    },
    Nothing,
    ErrUnableToInferReturnTypeForUnveiling,
    ErrEtherealSignature(EtherealSignatureError),
}

impl Unveiler {
    pub(crate) fn initialize_if_not(&mut self, return_ty: Option<EtherealTerm>, db: &::salsa::Db) {
        match self {
            Unveiler::Uninitialized => (),
            _ => return,
        }
        let Some(return_ty) = return_ty else {
            *self = Unveiler::ErrUnableToInferReturnTypeForUnveiling;
            return;
        };
        *self = match Self::new_aux(db, return_ty) {
            MaybeResult::JustOk(unveiler) => unveiler,
            MaybeResult::JustErr(e) => Unveiler::ErrEtherealSignature(e),
            MaybeResult::Nothing => Unveiler::Nothing,
        }
    }

    fn new_aux(db: &::salsa::Db, return_ty: EtherealTerm) -> EtherealSignatureMaybeResult<Self> {
        let templates = unveil_impl_block_signature_templates(db, return_ty)?;
        match templates.len() {
            0 => todo!(),
            1 => {
                let template = templates[0];
                if let Some(impl_block_signature) = template.try_into_signature(db) {
                    let unveil_output_ty_signature = template
                        .associated_output_template(db)?
                        .try_into_signature(db)
                        .expect("no generic parameters for Unveil::Output");
                    let unveil_output_ty = unveil_output_ty_signature.ty_term();
                    JustOk(Unveiler::UniqueFullyInstantiated {
                        opd_ty: impl_block_signature
                            .trai()
                            .application_expansion(db)
                            .arguments(db)[0],
                        unveil_output_ty,
                        unveil_output_ty_final_destination: unveil_output_ty.final_destination(db),
                        unveil_associated_fn_path: unveil_associated_fn_path(
                            &unveil_output_ty_signature,
                            db,
                        ),
                        unveil_output_ty_signature,
                    })
                } else {
                    JustOk(Unveiler::UniquePartiallyInstanted { template })
                }
            }
            _ => todo!(),
        }
    }
}

fn unveil_associated_fn_path(
    unveil_output_ty_signature: &TraitForTypeAssociatedTypeEtherealSignature,
    db: &::salsa::Db,
) -> TraitForTypeItemPath {
    let snake_case_unveil_ident = coword_menu(db).snake_case_unveil_ident();
    unveil_output_ty_signature
        .path()
        .impl_block(db)
        .associated_item_paths(db)
        .get_entry(snake_case_unveil_ident)
        .expect("unveil associated fn should exist!")
        .1
}

fn unveil_impl_block_signature_templates(
    db: &::salsa::Db,
    term: EtherealTerm,
) -> EtherealSignatureMaybeResult<&[TraitForTypeImplBlockEtherealSignatureBuilder]> {
    match term {
        EtherealTerm::Literal(_) => todo!(),
        EtherealTerm::Symbol(_) => {
            // ad hoc
            Nothing
        }
        EtherealTerm::Rune(_) => todo!(),
        EtherealTerm::EntityPath(path) => match path {
            TermEntityPath::Fugitive(_) => todo!(),
            TermEntityPath::Trait(_) => todo!(),
            TermEntityPath::TypeOntology(path) => {
                ty_ontology_path_unveil_impl_block_signature_templates(db, path).just_ok_as_ref2()
            }
            TermEntityPath::TypeInstance(_) => todo!(),
            TermEntityPath::TypeVariant(_) => todo!(),
        },
        EtherealTerm::Category(_) => todo!(),
        EtherealTerm::Universe(_) => todo!(),
        EtherealTerm::Curry(_) => todo!(),
        EtherealTerm::Ritchie(_) => todo!(),
        EtherealTerm::Abstraction(_) => todo!(),
        EtherealTerm::Application(path) => {
            ty_ontology_application_unveil_impl_block_signature_templates(db, path)
                .just_ok_as_ref2()
        }
        EtherealTerm::Subitem(_) => todo!(),
        EtherealTerm::AsTraitSubitem(_) => todo!(),
        EtherealTerm::TraitConstraint(_) => todo!(),
    }
}

#[salsa::tracked(jar = SemaExprJar, return_ref)]
fn ty_ontology_path_unveil_impl_block_signature_templates(
    db: &::salsa::Db,
    ty_path: TypePath,
) -> EtherealSignatureMaybeResult<SmallVec<[TraitForTypeImplBlockEtherealSignatureBuilder; 2]>> {
    unveil_impl_block_signature_templates_aux(
        db,
        ty_path,
        &[],
        EtherealTerm::EntityPath(TermEntityPath::TypeOntology(ty_path)),
    )
}

#[salsa::tracked(jar = SemaExprJar, return_ref)]
fn ty_ontology_application_unveil_impl_block_signature_templates(
    db: &::salsa::Db,
    ty_target: EtherealTermApplication,
) -> EtherealSignatureMaybeResult<SmallVec<[TraitForTypeImplBlockEtherealSignatureBuilder; 2]>> {
    let application_expansion = ty_target.application_expansion(db);
    let TermFunctionReduced::TypeOntology(ty_path) = application_expansion.function() else {
        todo!()
    };
    unveil_impl_block_signature_templates_aux(
        db,
        ty_path,
        application_expansion.arguments(db),
        ty_target.into(),
    )
}

fn unveil_impl_block_signature_templates_aux(
    db: &::salsa::Db,
    ty_path: TypePath,
    arguments: &[EtherealTerm],
    ty_target: EtherealTerm,
) -> EtherealSignatureMaybeResult<SmallVec<[TraitForTypeImplBlockEtherealSignatureBuilder; 2]>> {
    let item_path_menu = item_path_menu(db, ty_path.toolchain(db));
    let templates = ty_side_trai_for_ty_impl_block_signature_templates(
        db,
        item_path_menu.unveil_trai_path(),
        ty_path,
    )?;
    JustOk(
        templates
            .iter()
            .filter_map(|template| {
                template
                    .instantiate_ty(db, arguments, ty_target)
                    .into_option_result()
            })
            .collect::<EtherealSignatureResult<_>>()?,
    )
}
